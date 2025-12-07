use std::time::Duration;

/// Database configuration struct
/// # Fields
/// * `pg_uri` - PostgreSQL connection URI
/// * `connection_timeout` - Connection timeout duration
/// * `min_pool_size` - Minimum pool size
/// * `max_pool_size` - Maximum pool size
pub struct DatabaseConfig {
    pub pg_uri: String,
    pub connection_timeout: Option<Duration>,
    pub min_pool_size: Option<u32>,
    pub max_pool_size: Option<u32>,
}

impl DatabaseConfig {
    /// Create a new DatabaseConfig from environment variables
    pub fn new() -> Self {
        let pg_uri: String = std::env::var("DATABASE_URL")
            .expect("Failed to load `POSTGRES_URI` environment variable.");

        let connection_timeout: u64 = std::env::var("MONGO_CONNECTION_TIMEOUT")
            .unwrap_or("30".to_string())
            .parse()
            .expect("Failed to parse `CONNECTION_TIMEOUT` environment variable.");

        let min_pool_size: u32 = std::env::var("MONGO_MIN_POOL_SIZE")
            .unwrap_or("10".to_string())
            .parse()
            .expect("Failed to parse `MIN_POOL_SIZE` environment variable.");

        let max_pool_size: u32 = std::env::var("MONGO_MAX_POOL_SIZE")
            .unwrap_or("100".to_string())
            .parse()
            .expect("Failed to parse `MAX_POOL_SIZE` environment variable.");

        Self {
            pg_uri,
            connection_timeout: Some(Duration::from_secs(connection_timeout)),
            min_pool_size: Some(min_pool_size),
            max_pool_size: Some(max_pool_size),
        }
    }
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self::new()
    }
}
