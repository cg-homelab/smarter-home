use lib_models::error::Error;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::time::Duration;

pub mod config;
pub mod home;
pub mod power;
pub mod user;

#[derive(Clone)]
pub struct Db {
    pub pool: PgPool,
}
impl Db {
    async fn init(config: &config::DatabaseConfig) -> Result<PgPool, Error> {
        let db_connection_str = config.pg_uri.clone();
        // set up connection pool
        let pool = PgPoolOptions::new()
            .max_connections(config.max_pool_size.unwrap_or(100))
            .acquire_timeout(config.connection_timeout.unwrap_or(Duration::from_secs(30)))
            .connect(&db_connection_str)
            .await?;

        sqlx::migrate!("../../../migrations")
            .run(&pool)
            .await
            .map_err(|_error| Error::DbMigrationError)?;

        Ok(pool)
    }

    pub async fn new() -> Result<Self, Error> {
        let database_config = config::DatabaseConfig::new();

        let pg = Self::init(&database_config).await?;

        Ok(Self { pool: pg })
    }
    // async fn health_check(&self) -> Result<(), Error> {
    //     // simple query to check if db is reachable
    //     let _row: (i64,) = sqlx::query_as("SELECT 1").fetch_one(&self.pool).await?;
    //     Ok(())
    // }
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
