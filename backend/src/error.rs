use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("DB connection error: {0}")]
    DbConnection(String),

    #[error("DB error: {0}")]
    DbError(String),

    #[error("Generic error: {0}")]
    Generic(String),
}
