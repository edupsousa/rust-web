use crate::auth;
use crate::layout::template_response::TemplateResponse;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::{
    response::{IntoResponse, Redirect, Response},
    Form,
};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};

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

pub async fn get_login(Query(query): Query<NextUrl>) -> impl IntoResponse {
    TemplateResponse::new("auth/login")
        .content(LoginPageData {
            form: LoginForm::default(),
            errors: None,
            next_url: query.next,
        })
        .add_success_message("Please use the form above to login")
}

pub async fn post_login(
    mut auth_session: auth::layer::AuthSession,
    Query(NextUrl { next }): Query<NextUrl>,
    Form(form): Form<LoginForm>,
) -> Response {
    let template = TemplateResponse::new("auth/login");
    if let Err(errors) = form.validate() {
        return template
            .add_error_message("Please fix the errors above")
            .content(LoginPageData {
                form,
                errors: Some(errors),
                next_url: next,
            })
            .into_response();
    }
    let user = match auth_session.authenticate(form.clone().into()).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            return template
                .add_error_message("Invalid email or password")
                .content(LoginPageData {
                    form,
                    errors: None,
                    next_url: next,
                })
                .into_response();
        }
        Err(e) => {
            tracing::error!("Failed to authenticate user: {:?}", e);
            return template
                .add_error_message("Internal Error: Failed to authenticate user, try again later")
                .content(LoginPageData {
                    form,
                    errors: None,
                    next_url: next,
                })
                .into_response();
        }
    };

    if auth_session.login(&user).await.is_err() {
        tracing::error!("Failed to login user: {:?}", user);
        return template
            .add_error_message("Internal Error: Failed to login user, try again later")
            .content(LoginPageData {
                form,
                errors: None,
                next_url: next,
            })
            .into_response();
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
