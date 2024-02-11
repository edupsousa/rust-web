use std::collections::HashMap;

use crate::{
    templates::{render_response, TemplateEngine},
    services::user_service,
};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
    Extension, Form,
};
use sea_orm::DatabaseConnection;
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

impl From<RegisterForm> for user_service::CreateUserData {
    fn from(form: RegisterForm) -> Self {
        user_service::CreateUserData {
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

pub async fn get(Extension(template_engine): Extension<TemplateEngine>) -> Response {
    render_response(
        &template_engine,
        "user/register",
        &RegisterPageData::default(),
    )
}

pub async fn post(
    Extension(template_engine): Extension<TemplateEngine>,
    Extension(db): Extension<DatabaseConnection>,
    Form(form): Form<RegisterForm>,
) -> Response {
    let mut errors: FormErrors = form.get_errors();
    if errors.is_empty() {
        if user_service::user_exists(&db, &form.email).await {
            errors.insert("email", "Email is already registered");
        } else {
            match user_service::create_user(&db, form.into()).await {
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
    render_response(&template_engine, "user/register", &data)
}

