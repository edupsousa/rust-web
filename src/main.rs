mod routes;
mod templates;
mod database;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    dotenvy::dotenv().unwrap();

    let database_connection = database::connect().await;
    let template_engine = templates::create_engine();
    let app = routes::create_router(template_engine, database_connection);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::debug!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}