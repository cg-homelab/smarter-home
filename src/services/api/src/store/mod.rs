use surrealdb::{engine::remote::ws::Ws, opt::auth::Root};

pub mod home;
pub mod power;
pub mod user;

use crate::error::Error;
use crate::DB;

pub async fn init() -> Result<String, Error> {
    let db_connection_str =
        std::env::var("SURREALDB_URL").unwrap_or_else(|_| "localhost:8000".to_string());
    let db_user = std::env::var("SURREALDB_USER").unwrap_or_else(|_| "root".to_string());
    let db_pass = std::env::var("SURREALDB_PASS").unwrap_or_else(|_| "root".to_string());
    let db_name = std::env::var("SURREALDB_DB").unwrap_or_else(|_| "test".to_string());
    let db_ns = std::env::var("SURREALDB_NS").unwrap_or_else(|_| "test".to_string());

    DB.connect::<Ws>(db_connection_str.as_str()).await?;

    DB.signin(Root {
        username: &db_user,
        password: &db_pass,
    })
    .await?;

    // select a specific namespace / database
    DB.use_ns(db_ns).use_db(db_name).await.unwrap();

    Ok("SurrealDB connected".to_string())
}
