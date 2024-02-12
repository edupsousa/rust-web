use std::collections::HashMap;

use super::db_user;
use crate::app::AppState;
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
    Form,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct RegisterForm {
    email: String,
    password: String,
    confirm_password: String,
}

type FormErrors = HashMap<&'static str, &'static str>;

impl RegisterForm {
    fn get_errors(&self) -> FormErrors {
        let mut errors = FormErrors::default();

        if self.email.is_empty() {
            errors.insert("email", "Email is required");
        } else if !validator::validate_email(&self.email) {
            errors.insert("email", "Invalid email address");
        }

        if self.password.len() < 8 {
            errors.insert("password", "Password must be at least 8 characters long");
        }

        if self.confirm_password.is_empty() {
            errors.insert("confirm_password", "Confirm password is required");
        } else if self.password != self.confirm_password {
            errors.insert("confirm_password", "Passwords do not match");
        }

        errors
    }
}

impl From<RegisterForm> for db_user::CreateUserData {
    fn from(form: RegisterForm) -> Self {
        db_user::CreateUserData {
            email: form.email,
            password: form.password,
        }
    }
}

#[derive(Serialize, Default, Debug)]
pub struct RegisterPageData {
    form: RegisterForm,
    errors: FormErrors,
}

pub async fn get_register(State(app): State<AppState>) -> Response {
    app.template_engine
        .render_response("user/register", &RegisterPageData::default())
}

pub async fn post_register(
    State(app): State<AppState>,
    Form(form): Form<RegisterForm>,
) -> Response {
    let mut errors: FormErrors = form.get_errors();
    if errors.is_empty() {
        if db_user::user_exists(&app.database_connection, &form.email).await {
            errors.insert("email", "Email is already registered");
        } else {
            match db_user::create_user(&app.database_connection, form.into()).await {
                Ok(_) => {
                    return Redirect::to("/login?registered=true").into_response();
                }
                Err(e) => {
                    tracing::error!("Failed to create user: {:?}", e);
                    return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create user")
                        .into_response();
                }
            }
        }
    }
    let data = RegisterPageData { form, errors };
    app.template_engine.render_response("user/register", &data)
}
