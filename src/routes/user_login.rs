use std::collections::HashMap;

use crate::templates::{render_response, TemplateEngine};
use crate::password;
use axum::{
    response::{IntoResponse, Redirect, Response}, Extension, Form
};
use entity::user::{Column, Entity as User, Model as UserModel};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use validator::validate_email;

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct LoginForm {
    email: String,
    password: String,
}

type FormErrors = HashMap<&'static str, &'static str>;

impl LoginForm {
    fn get_errors(&self) -> FormErrors {
        let mut errors = FormErrors::default();

        if self.email.is_empty() {
            errors.insert("email", "Email is required");
        } else if !validate_email(&self.email) {
            errors.insert("email", "Invalid email address");
        }

        if self.password.is_empty() {
            errors.insert("password", "Password is required");
        }

        errors
    }
}

#[derive(Serialize, Default, Debug)]
pub struct LoginPageData {
    form: LoginForm,
    errors: FormErrors,
}

pub async fn get(Extension(template_engine): Extension<TemplateEngine>) -> Response {
    render_response(
        &template_engine,
        "user/login",
        &LoginPageData::default(),
    )
}

pub async fn post(
    Extension(template_engine): Extension<TemplateEngine>,
    Extension(db): Extension<DatabaseConnection>,
    Form(form): Form<LoginForm>,
) -> Response {
    let mut errors: FormErrors = form.get_errors();
    if errors.is_empty() {
        let user = authenticate_user(&db, &form.email, &form.password).await;
        if user.is_none() {
            errors.insert("password", "Invalid email or password");
        } else {
          return Redirect::to("/").into_response();
        }
    }

    let data = LoginPageData {
        form,
        errors,
    };
    render_response(&template_engine, "user/login", &data)
}

async fn authenticate_user(
    db: &DatabaseConnection,
    email: &str,
    password: &str,
) -> Option<UserModel> {
    User::find()
        .filter(Column::Email.contains(email))
        .one(db)
        .await
        .ok()
        .flatten()
        .filter(|user| password::verify(password, &user.password))
}