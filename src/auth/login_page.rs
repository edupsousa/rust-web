use crate::app::AppState;
use crate::auth;
use crate::layout::messages::PageMessages;
use crate::layout::page_template::PageTemplate;
use crate::templates::TemplateEngine;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::{
    response::{IntoResponse, Redirect, Response},
    Form,
};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};

use super::layer::AuthSession;

#[derive(Deserialize, Serialize, Default, Debug, Clone, Validate)]
pub struct LoginForm {
    #[validate(email(message = "Invalid email address"))]
    email: String,
    #[validate(length(min = 1, message = "Password is required"))]
    password: String,
}

impl From<LoginForm> for auth::layer::Credentials {
    fn from(form: LoginForm) -> Self {
        auth::layer::Credentials {
            email: form.email,
            password: form.password,
        }
    }
}

#[derive(Serialize, Default, Debug)]
pub struct LoginPageData {
    form: LoginForm,
    errors: Option<ValidationErrors>,
    next_url: Option<String>,
}

#[derive(Deserialize)]
pub struct NextUrl {
    next: Option<String>,
}

pub fn render_login_page(
    template_engine: &TemplateEngine,
    is_signed_in: bool,
    next: Option<String>,
    form: LoginForm,
    errors: Option<ValidationErrors>,
    messages: Option<PageMessages>,
) -> Response {
    PageTemplate::builder("auth/login")
        .content(LoginPageData {
            form,
            errors,
            next_url: next,
        })
        .maybe_messages(messages)
        .navbar(is_signed_in)
        .build()
        .render(template_engine)
}

pub async fn get_login(
    State(app): State<AppState>,
    auth_session: AuthSession,
    Query(NextUrl { next }): Query<NextUrl>,
) -> Response {
    render_login_page(
        &app.template_engine,
        auth_session.user.is_some(),
        next,
        LoginForm::default(),
        None,
        None,
    )
}

pub async fn post_login(
    mut auth_session: auth::layer::AuthSession,
    State(app): State<AppState>,
    Query(NextUrl { next }): Query<NextUrl>,
    Form(form): Form<LoginForm>,
) -> Response {
    if let Err(errors) = form.validate() {
        return render_login_page(
            &app.template_engine,
            auth_session.user.is_some(),
            next,
            form,
            Some(errors),
            None,
        );
    }
    let user = match auth_session.authenticate(form.clone().into()).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            let mut messages = PageMessages::new();
            messages.error("Invalid email or password");
            return render_login_page(
                &app.template_engine,
                auth_session.user.is_some(),
                next,
                form,
                None,
                Some(messages),
            );
        }
        Err(e) => {
            tracing::error!("Failed to authenticate user: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to authenticate user",
            )
                .into_response();
        }
    };

    if auth_session.login(&user).await.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to login").into_response();
    }

    if let Some(next) = next {
        return Redirect::to(&next).into_response();
    }

    Redirect::to("/").into_response()
}

pub async fn get_logout(mut auth_session: auth::layer::AuthSession) -> Response {
    match auth_session.logout().await {
        Ok(_) => Redirect::to("/login").into_response(),
        Err(e) => {
            tracing::error!("Failed to logout: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to logout").into_response()
        }
    }
}
