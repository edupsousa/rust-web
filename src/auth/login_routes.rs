use crate::app::AppState;
use crate::auth;
use crate::templates::{render_to_response, TemplateEngine};
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::{
    response::{IntoResponse, Redirect, Response},
    Form,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use validator::validate_email;

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
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
    errors: FormErrors,
    next_url: Option<String>,
}

#[derive(Deserialize)]
pub struct NextUrl {
    next: Option<String>,
}

pub fn render_login_page(
    template_engine: &TemplateEngine,
    next: Option<String>,
    form: LoginForm,
    errors: FormErrors,
) -> Response {
    let data = LoginPageData {
        next_url: next,
        form,
        errors,
    };
    render_to_response(template_engine, "user/login", &data)
}

pub async fn get_login(
    State(app): State<AppState>,
    Query(NextUrl { next }): Query<NextUrl>,
) -> Response {
    render_login_page(
        &app.template_engine,
        next,
        LoginForm::default(),
        FormErrors::default(),
    )
}

pub async fn post_login(
    mut auth_session: auth::layer::AuthSession,
    State(app): State<AppState>,
    Query(NextUrl { next }): Query<NextUrl>,
    Form(form): Form<LoginForm>,
) -> Response {
    let mut errors: FormErrors = form.get_errors();
    if !errors.is_empty() {
        return render_login_page(&app.template_engine, next, form, errors);
    }
    let user = match auth_session.authenticate(form.clone().into()).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            errors.insert("password", "Invalid email or password");
            return render_login_page(&app.template_engine, next, form, errors);
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
