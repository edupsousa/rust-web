use crate::app::AppState;

use super::{
    login_routes::{get_login, get_logout, post_login},
    register_routes::{get_register, post_register},
};
use axum::{
    routing::{get, post},
    Router,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/login", get(get_login))
        .route("/login", post(post_login))
        .route("/logout", get(get_logout))
        .route("/register", get(get_register))
        .route("/register", post(post_register))
}
