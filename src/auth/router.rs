use axum::{routing::{get, post}, Router};
use super::{login_routes::{get_login, post_login}, register_routes::{get_register, post_register}};

pub fn router() -> Router {
  Router::new()
    .route("/login", get(get_login))
    .route("/login", post(post_login))
    .route("/register", get(get_register))
    .route("/register", post(post_register))
}
