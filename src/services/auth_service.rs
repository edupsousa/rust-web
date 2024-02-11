use argon2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Argon2,
};
use async_trait::async_trait;
use axum_login::{AuthUser, AuthnBackend, UserId};
use entity::user;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

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
            .filter(|user| verify(&credentials.password, &user.password));

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

fn verify(password: &str, hash: &str) -> bool {
    let argon2 = Argon2::default();
    let password_hash = PasswordHash::new(hash).unwrap();
    argon2
        .verify_password(password.as_bytes(), &password_hash)
        .is_ok()
}

pub type AuthSession = axum_login::AuthSession<Backend>;
