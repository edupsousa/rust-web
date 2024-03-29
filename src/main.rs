mod app;
mod auth;
mod database;
mod layout;
mod templates;
mod user;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    dotenvy::dotenv().unwrap();

    let database_connection = database::connect().await;
    let template_engine = templates::build_template_engine().unwrap();
    let app = app::create_app(template_engine, database_connection);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::debug!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
