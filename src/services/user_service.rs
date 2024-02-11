use entity::user;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use thiserror::Error;
use argon2::{
    password_hash::{
        Error,
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
  };

pub async fn user_exists(db: &DatabaseConnection, email: &str) -> bool {
    user::Entity::find()
        .filter(user::Column::Email.eq(email))
        .one(db)
        .await
        .ok()
        .flatten()
        .is_some()
}

#[derive(Error, Debug)]
pub enum CreateUserError {
    #[error("Failed to hash password")]
    HashPassword(HashError),
    #[error("Failed to save user")]
    SaveUser(#[from] sea_orm::error::DbErr),
}

pub struct CreateUserData {
    pub email: String,
    pub password: String,
}

pub async fn create_user(
    db: &DatabaseConnection,
    data: CreateUserData,
) -> Result<user::ActiveModel, CreateUserError> {
    let hashed_password = hash(&data.password).map_err(CreateUserError::HashPassword)?;

    user::ActiveModel {
        email: Set(data.email),
        password: Set(hashed_password),
        ..Default::default()
    }
    .save(db)
    .await
    .map_err(CreateUserError::SaveUser)
}

pub async fn authenticate_user(
    db: &DatabaseConnection,
    email: &str,
    password: &str,
) -> Option<user::Model> {
    user::Entity::find()
        .filter(user::Column::Email.contains(email))
        .one(db)
        .await
        .ok()
        .flatten()
        .filter(|user| verify(password, &user.password))
}

type HashError = Error;

fn hash(password: &str) -> Result<String, Error> {
  let salt = SaltString::generate(&mut OsRng);
  let argon2 = Argon2::default();
  let password_hash = argon2.hash_password(password.as_bytes(), &salt)?.to_string();
  
  Ok(password_hash)
}

fn verify(password: &str, hash: &str) -> bool {
  let argon2 = Argon2::default();
  let password_hash = PasswordHash::new(hash).unwrap();
  argon2.verify_password(password.as_bytes(), &password_hash).is_ok()
}