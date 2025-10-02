use lib_models::error::Error;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::time::Duration;

pub mod config;
pub mod home;
pub mod power;
pub mod user;

const MONGO_DB: &str = "smarter-home";

#[derive(Clone)]
pub struct DatabaseState {
    pub pg: PgPool,
}
impl DatabaseState {
    async fn init_pg(config: &config::DatabaseConfig) -> Result<PgPool, Error> {
        let db_connection_str = config.pg_uri.clone();
        // set up connection pool
        Ok(PgPoolOptions::new()
            .max_connections(config.max_pool_size.unwrap_or(100))
            .acquire_timeout(config.connection_timeout.unwrap_or(Duration::from_secs(30)))
            .connect(&db_connection_str)
            .await?)
    }

    pub async fn new() -> Result<Self, Error> {
        let database_config = config::DatabaseConfig::new();

        let pg = Self::init_pg(&database_config).await?;

        Ok(Self { pg })
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
