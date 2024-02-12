use async_trait::async_trait;
use axum_login::{AuthManagerLayer, AuthManagerLayerBuilder, AuthUser, AuthnBackend, UserId};
use sea_orm::DatabaseConnection;
use tower_sessions::{cookie::time::Duration, Expiry, SessionManagerLayer};

use super::{db_session_store::DatabaseSessionStore, db_user, password};

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

impl From<db_user::UserModel> for User {
    fn from(user: db_user::UserModel) -> Self {
        Self {
            id: user.id,
            pw_hash: user.password.as_bytes().to_vec(),
        }
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
        let user = db_user::get_user_by_email(&self.db, &credentials.email)
            .await
            .filter(|user| password::verify(&credentials.password, &user.password))
            .map(User::from);

        Ok(user)
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        let user = db_user::get_user_by_id(&self.db, *user_id)
            .await
            .map(User::from);

        Ok(user)
    }
}

pub type AuthSession = axum_login::AuthSession<Backend>;

pub fn create_auth_layer(
    db: DatabaseConnection,
) -> AuthManagerLayer<Backend, DatabaseSessionStore> {
    let session_store = DatabaseSessionStore::new(db.clone());
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::minutes(30)));
    let backend = Backend::new(db);

    AuthManagerLayerBuilder::new(backend, session_layer).build()
}
