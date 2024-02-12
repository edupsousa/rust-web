use std::env;

use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};


pub async fn connect() -> DatabaseConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = Database::connect(database_url).await.expect("Failed to connect to database");
    Migrator::up(&conn, None).await.expect("Failed to run migrations");

    conn
}