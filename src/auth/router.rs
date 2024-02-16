use crate::app::AppState;

use super::{
    layer::AuthSession,
    login_page::{get_login, get_logout, post_login},
    register_page::{get_register, post_register},
};
use axum::{
    extract::Request,
    middleware::{self, Next},
    response::{IntoResponse, Redirect, Response},
    routing::{get, post},
    Router,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/login", get(get_login))
        .route("/login", post(post_login))
        .route("/register", get(get_register))
        .route("/register", post(post_register))
        .layer(middleware::from_fn(invalid_when_signed_in))
        .route("/logout", get(get_logout))
}

async fn invalid_when_signed_in(
    auth_session: AuthSession,
    request: Request,
    next: Next,
) -> Response {
    if auth_session.user.is_some() {
        return Redirect::to("/").into_response();
    }

    next.run(request).await
}
