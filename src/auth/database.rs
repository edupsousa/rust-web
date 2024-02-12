use entity::user;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use thiserror::Error;

use super::password;

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
    HashPassword(password::HashError),
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
    let hashed_password = password::hash(&data.password).map_err(CreateUserError::HashPassword)?;

    user::ActiveModel {
        email: Set(data.email),
        password: Set(hashed_password),
        ..Default::default()
    }
    .save(db)
    .await
    .map_err(CreateUserError::SaveUser)
}
