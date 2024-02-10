use std::collections::HashMap;

use crate::templates::{render_response, TemplateEngine};
use axum::{
    response::{IntoResponse, Redirect, Response},
    Extension, Form,
};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
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
    let errors: FormErrors = form.get_errors();
    if errors.is_empty() {
        create_user(&db, &form).await;
        return Redirect::to("/user/login?registered=true").into_response();
    }
    let data = RegisterPageData { form, errors };
    render_response(&template_engine, "user/register", &data)
}

use entity::user;

pub async fn create_user(db: &DatabaseConnection, form: &RegisterForm) {
    user::ActiveModel {
        email: Set(form.email.to_owned()),
        password: Set(form.password.to_owned()),
        ..Default::default()
    }
    .save(db)
    .await
    .expect("Failed to save user");
}
