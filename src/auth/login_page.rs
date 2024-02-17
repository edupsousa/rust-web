use crate::auth;
use crate::layout::layout_middleware::LayoutMiddleware;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::Extension;
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

pub async fn get_login(
    Extension(mut layout): Extension<LayoutMiddleware>,
    Query(query): Query<NextUrl>,
) -> Response {
    layout.add_success_message("Please use the form above to login");
    layout.render(
        "auth/login",
        LoginPageData {
            form: LoginForm::default(),
            errors: None,
            next_url: query.next,
        },
    )
}

pub async fn post_login(
    Extension(mut layout): Extension<LayoutMiddleware>,
    mut auth_session: auth::layer::AuthSession,
    Query(NextUrl { next }): Query<NextUrl>,
    Form(form): Form<LoginForm>,
) -> Response {
    if let Err(errors) = form.validate() {
        layout.add_error_message("Please fix the errors above");
        return layout.render(
            "auth/login",
            LoginPageData {
                form,
                errors: Some(errors),
                next_url: next,
            },
        );
    }
    let user = match auth_session.authenticate(form.clone().into()).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            layout.add_error_message("Invalid email or password");
            return layout.render(
                "auth/login",
                LoginPageData {
                    form,
                    errors: None,
                    next_url: next,
                },
            );
        }
        Err(e) => {
            tracing::error!("Failed to authenticate user: {:?}", e);
            layout
                .add_error_message("Internal Error: Failed to authenticate user, try again later");
            return layout.render(
                "auth/login",
                LoginPageData {
                    form,
                    errors: None,
                    next_url: next,
                },
            );
        }
    };

    if auth_session.login(&user).await.is_err() {
        tracing::error!("Failed to login user: {:?}", user);
        layout.add_error_message("Internal Error: Failed to login user, try again later");
        return layout.render(
            "auth/login",
            LoginPageData {
                form,
                errors: None,
                next_url: next,
            },
        );
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
