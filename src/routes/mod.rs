mod user_register;

use axum::{
    response::Response,
    routing::{get, post},
    Extension, Router,
};
use axum_login::{login_required, tower_sessions::{cookie::time::Duration, Expiry, SessionManagerLayer}, AuthManagerLayerBuilder};
use sea_orm::DatabaseConnection;
use tower_http::trace::TraceLayer;

use crate::{services::auth_service, templates::{render_response, TemplateEngine}};

pub fn create_router(
    template_engine: TemplateEngine,
    database_connection: DatabaseConnection,
) -> Router {
    let session_store = tower_sessions::MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::minutes(30)));
    let backend = auth_service::Backend::new(database_connection.clone());
    let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

    let auth_router = crate::auth::router::router();

    Router::new()
        .route("/", get(get_root))
        .route_layer(login_required!(auth_service::Backend, login_url = "/login"))
        .route("/user/register", get(user_register::get))
        .route("/user/register", post(user_register::post))
        .merge(auth_router)
        .layer(Extension(template_engine))
        .layer(Extension(database_connection))
        .layer(auth_layer)
        .layer(TraceLayer::new_for_http())
}

pub async fn get_root(Extension(template_engine): Extension<TemplateEngine>) -> Response {
    render_response(&template_engine, "index", &())
}
