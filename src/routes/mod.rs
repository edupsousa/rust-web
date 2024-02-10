mod user_register;

use axum::{
    response::Response,
    routing::{get, post},
    Extension, Router,
};
use sea_orm::DatabaseConnection;
use tower_http::trace::TraceLayer;

use crate::templates::{render_response, TemplateEngine};

pub fn create_router(
    template_engine: TemplateEngine,
    database_connection: DatabaseConnection,
) -> Router {
    Router::new()
        .route("/", get(get_root))
        .route("/user/register", get(user_register::get))
        .route("/user/register", post(user_register::post))
        .layer(Extension(template_engine))
        .layer(Extension(database_connection))
        .layer(TraceLayer::new_for_http())
}

pub async fn get_root(Extension(template_engine): Extension<TemplateEngine>) -> Response {
    render_response(&template_engine, "index", &())
}
