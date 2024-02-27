use diesel::{Connection, ConnectionError, PgConnection};
use std::env;

use crate::error::ApiError;

pub mod user;

pub struct ModelManager {
    connection_string: String,
}
impl ModelManager {
    pub fn new() -> Self {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        Self {
            connection_string: database_url,
        }
    }
    pub fn establish_connection(&self) -> Result<PgConnection, ApiError> {
        let connection =
            PgConnection::establish(&self.connection_string).map_err(from_connection_error)?;
        Ok(connection)
    }
}

pub(super) fn from_connection_error(err: ConnectionError) -> ApiError {
    match err {
        ConnectionError::InvalidCString(_) => {
            ApiError::DbConnection("The connection URL contained a `NUL` byte".to_string())
        }
        ConnectionError::BadConnection(val) => {
            ApiError::DbConnection(format!("The database returned an error: {0}", val).to_string())
        }
        ConnectionError::InvalidConnectionUrl(val) => ApiError::DbConnection(
            format!("The connection URL could not be parsed: {0}", val).to_string(),
        ),
        ConnectionError::CouldntSetupConfiguration(val) => ApiError::DbConnection(
            format!(
                "Diesel could not configure the database connection: {0}",
                val
            )
            .to_string(),
        ),
        _ => ApiError::DbConnection("unspecified error".to_string()),
    }
}
pub(super) fn from_diesel_error(err: diesel::result::Error) -> ApiError {
    match err {
        diesel::result::Error::InvalidCString(_) => {
            ApiError::DbConnection("DBError: The connection URL contained a `NUL` byte".to_string())
        }
        diesel::result::Error::DatabaseError(val_kind, val) => {
            ApiError::DbError(format!("DBError: {:?} - {:?}", val_kind, val).to_string())
        }
        diesel::result::Error::NotFound => ApiError::DbError("DBError: Not found".to_string()),
        diesel::result::Error::QueryBuilderError(val) => {
            ApiError::DbError(format!("DBError: {0}", val).to_string())
        }
        diesel::result::Error::DeserializationError(val) => {
            ApiError::DbError(format!("DBError: {0}", val).to_string())
        }
        diesel::result::Error::SerializationError(val) => {
            ApiError::DbError(format!("DBError: {0}", val).to_string())
        }
        diesel::result::Error::RollbackErrorOnCommit {
            rollback_error: _,
            commit_error: _,
        } => ApiError::DbError("DBError: Rollback error on commit".to_string()),
        diesel::result::Error::RollbackTransaction => {
            ApiError::DbError("DBError: Rollback transaction".to_string())
        }
        diesel::result::Error::AlreadyInTransaction => {
            ApiError::DbError("DBError: Already in transaction".to_string())
        }
        diesel::result::Error::NotInTransaction => {
            ApiError::DbError("DBError: Not in transaction".to_string())
        }
        diesel::result::Error::BrokenTransactionManager => {
            ApiError::DbError("DBError: Broken transaction".to_string())
        }
        _ => ApiError::DbError("DBError: Generic".to_string()),
    }
}
