use super::{db_user, layer::AuthSession};
use crate::{
    app::AppState,
    layout::{messages::PageMessages, page_template::PageTemplate},
    templates::TemplateEngine,
};
use axum::{
    extract::State,
    http::StatusCode,
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

pub async fn get_register(State(app): State<AppState>, auth_session: AuthSession) -> Response {
    render_register_page(
        &app.template_engine,
        RegisterPageData::default(),
        auth_session.user.is_some(),
        None,
    )
}

pub async fn post_register(
    State(app): State<AppState>,
    auth_session: AuthSession,
    Form(form): Form<RegisterForm>,
) -> Response {
    if let Err(errors) = form.validate() {
        return render_register_page(
            &app.template_engine,
            RegisterPageData {
                form,
                errors: Some(errors),
            },
            auth_session.user.is_some(),
            None,
        );
    }

    if db_user::user_exists(&app.database_connection, &form.email).await {
        let mut messages = PageMessages::new();
        messages.error("User already exists");
        return render_register_page(
            &app.template_engine,
            RegisterPageData { form, errors: None },
            auth_session.user.is_some(),
            Some(messages),
        );
    }

    match db_user::create_user(&app.database_connection, form.into()).await {
        Ok(_) => Redirect::to("/login?registered=true").into_response(),
        Err(e) => {
            tracing::error!("Failed to create user: {:?}", e);

            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create user").into_response()
        }
    }
}

fn render_register_page(
    template_engine: &TemplateEngine,
    data: RegisterPageData,
    is_signed_in: bool,
    messages: Option<PageMessages>,
) -> Response {
    PageTemplate::builder("auth/register")
        .content(data)
        .navbar(is_signed_in)
        .maybe_messages(messages)
        .build()
        .render(template_engine)
}
