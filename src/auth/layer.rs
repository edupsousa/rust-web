use async_trait::async_trait;
use axum_login::{AuthManagerLayer, AuthManagerLayerBuilder, AuthUser, AuthnBackend, UserId};
use entity::user;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use tower_sessions::{cookie::time::Duration, Expiry, SessionManagerLayer};

use super::password;

#[derive(Debug, Clone)]
pub struct User {
  id: i32,
  pw_hash: Vec<u8>,
}

impl AuthUser for User {
  type Id = i32;

  fn id(&self) -> Self::Id {
      self.id
  }

  fn session_auth_hash(&self) -> &[u8] {
      &self.pw_hash
  }
}

#[derive(Clone)]
pub struct Backend {
  db: DatabaseConnection,
}

impl Backend {
  pub fn new(db: DatabaseConnection) -> Self {
      Self { db }
  }
}

pub struct Credentials {
  pub email: String,
  pub password: String,
}

#[async_trait]
impl AuthnBackend for Backend {
  type User = User;
  type Credentials = Credentials;
  type Error = std::convert::Infallible;

  async fn authenticate(
      &self,
      credentials: Self::Credentials,
  ) -> Result<Option<Self::User>, Self::Error> {
      let user = user::Entity::find()
          .filter(user::Column::Email.contains(&credentials.email))
          .one(&self.db)
          .await
          .ok()
          .flatten()
          .filter(|user| password::verify(&credentials.password, &user.password));

      let user = user.map(|user| User {
          id: user.id,
          pw_hash: user.password.as_bytes().to_vec(),
      });

      Ok(user)
  }

  async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
      let user = user::Entity::find_by_id(*user_id).one(&self.db).await;

      match user {
          Ok(Some(user)) => Ok(Some(User {
              id: user.id,
              pw_hash: user.password.as_bytes().to_vec(),
          })),
          _ => Ok(None),
      }
  }
}

pub type AuthSession = axum_login::AuthSession<Backend>;

pub fn create_auth_layer(db: DatabaseConnection) -> AuthManagerLayer<Backend, tower_sessions::MemoryStore> {
  let session_store = tower_sessions::MemoryStore::default();
  let session_layer = SessionManagerLayer::new(session_store)
      .with_secure(false)
      .with_expiry(Expiry::OnInactivity(Duration::minutes(30)));
  let backend = Backend::new(db);
  
  AuthManagerLayerBuilder::new(backend, session_layer).build()
}