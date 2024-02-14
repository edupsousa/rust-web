use axum::{extract::State, response::Response, routing::get, Router};
use axum_login::login_required;
use sea_orm::DatabaseConnection;
use serde::Serialize;
use tower_http::trace::TraceLayer;

use crate::{
    auth::{self, layer::AuthSession},
    templates::{render_to_response, TemplateEngine},
};

#[derive(Clone)]
pub struct AppState {
    pub template_engine: TemplateEngine,
    pub database_connection: DatabaseConnection,
}

pub fn create_app(
    template_engine: TemplateEngine,
    database_connection: DatabaseConnection,
) -> Router {
    let auth_layer = auth::layer::create_auth_layer(database_connection.clone());
    let auth_router = crate::auth::router::router();

    let app_state = AppState {
        template_engine,
        database_connection,
    };

    Router::new()
        .route("/protected", get(get_protected))
        .route_layer(login_required!(auth::layer::Backend, login_url = "/login"))
        .merge(auth_router)
        .route("/public", get(get_public))
        .route("/", get(get_root))
        .layer(auth_layer)
        .layer(TraceLayer::new_for_http())
        .with_state(app_state)
}

#[derive(Serialize)]
struct IndexTemplate {
    navbar: NavbarTemplate,
}

#[derive(Serialize)]
struct NavbarTemplate {
    login_visible: bool,
    signup_visible: bool,
    logout_visible: bool,
    private_visible: bool,
}

pub async fn get_root(State(app): State<AppState>, auth_session: AuthSession) -> Response {
    let is_signed_in = auth_session.user.is_some();
    let navbar = NavbarTemplate {
        login_visible: !is_signed_in,
        signup_visible: !is_signed_in,
        logout_visible: is_signed_in,
        private_visible: is_signed_in,
    };
    let index = IndexTemplate { navbar };
    render_to_response(&app.template_engine, "index", &index)
}

pub async fn get_protected() -> &'static str {
    "Protected"
}

pub async fn get_public() -> &'static str {
    "Public"
}
