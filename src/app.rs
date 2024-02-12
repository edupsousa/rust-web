use axum::{response::Response, routing::get, Extension, Router};
use axum_login::login_required;
use sea_orm::DatabaseConnection;
use tower_http::trace::TraceLayer;

use crate::{
    auth,
    templates::{render_response, TemplateEngine},
};

pub fn create_app(
    template_engine: TemplateEngine,
    database_connection: DatabaseConnection,
) -> Router {
    let auth_layer = auth::layer::create_auth_layer(database_connection.clone());
    let auth_router = crate::auth::router::router();

    Router::new()
        .route("/protected", get(get_protected))
        .route_layer(login_required!(auth::layer::Backend, login_url = "/login"))
        .merge(auth_router)
        .route("/public", get(get_public))
        .route("/", get(get_root))
        .layer(Extension(template_engine))
        .layer(Extension(database_connection))
        .layer(auth_layer)
        .layer(TraceLayer::new_for_http())
}

pub async fn get_root(Extension(template_engine): Extension<TemplateEngine>) -> Response {
    render_response(&template_engine, "index", &())
}

pub async fn get_protected() -> &'static str {
    "Protected"
}

pub async fn get_public() -> &'static str {
    "Public"
}
