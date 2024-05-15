use std::time::Duration;

use sqlx::{postgres::PgPoolOptions, PgPool};

pub mod power;
pub mod user;

pub async fn init() -> PgPool {
    let db_connection_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost".to_string());

    // set up connection pool
    PgPoolOptions::new()
        .max_connections(30)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database")
}
