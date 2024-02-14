use super::profile_page;
use crate::app::AppState;
use axum::{
    routing::{get, post},
    Router,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/user/profile", get(profile_page::get_profile_page))
        .route("/user/profile", post(profile_page::post_profile_page))
}
