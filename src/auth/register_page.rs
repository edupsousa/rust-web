use super::db_user;
use crate::{app::AppState, layout::template_response::TemplateResponse};
use axum::{
    extract::State,
    response::{IntoResponse, Redirect, Response},
    Form,
};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};

#[derive(Debug, Deserialize, Serialize, Default, Validate)]
pub struct RegisterForm {
    #[validate(email(message = "Invalid email address"))]
    email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    password: String,
    #[validate(must_match(other = "password", message = "Passwords do not match"))]
    confirm_password: String,
}

impl From<RegisterForm> for db_user::CreateUserData {
    fn from(form: RegisterForm) -> Self {
        db_user::CreateUserData {
            email: form.email,
            password: form.password,
        }
    }
}

#[derive(Debug, Default, Serialize)]
pub struct RegisterPageData {
    form: RegisterForm,
    errors: Option<ValidationErrors>,
}

pub async fn get_register() -> Response {
    TemplateResponse::new("auth/register")
        .content(RegisterPageData::default())
        .into_response()
}

pub async fn post_register(
    State(app): State<AppState>,
    Form(form): Form<RegisterForm>,
) -> Response {
    let response = TemplateResponse::new("auth/register");
    if let Err(errors) = form.validate() {
        return response
            .content(RegisterPageData {
                form,
                errors: Some(errors),
            })
            .into_response();
    }

    if db_user::user_exists(&app.database_connection, &form.email).await {
        return response
            .content(RegisterPageData { form, errors: None })
            .add_error_message("User already exists")
            .into_response();
    }

    match db_user::create_user(&app.database_connection, form.into()).await {
        Ok(_) => Redirect::to("/login?registered=true").into_response(),
        Err(e) => {
            tracing::error!("Failed to create user: {:?}", e);

            response
                .content(RegisterPageData::default())
                .add_error_message("Failed to create user")
                .into_response()
        }
    }
}
