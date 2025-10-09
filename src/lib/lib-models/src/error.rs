use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::Json;
// use mongodb::bson::ser::Error as BsonError;
// use mongodb::error as MongodbError;
// use questdb::Error as QuestDbError;
use sqlx::Error as SqlxError;
// use surrealdb::Error as SurrealError;
use thiserror::Error;
use tokio::sync::mpsc::error::TrySendError;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Internal Server Error")]
    InternalServerError,

    #[error("database error")]
    Db,

    #[error("database returned no rows")]
    DbReturnedNoRows,

    #[error("db migration error")]
    DbMigrationError,

    #[error("server error")]
    AxumServerError,

    #[error("model conversion error")]
    ModelConversionError,

    #[error("not found")]
    EntityNotFound,

    #[error("wrong password")]
    WrongPassword,

    #[error("crypto hash error")]
    CryptoHashError,

    #[error("conflict: {0}")]
    Conflict(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let status = match &self {
            Error::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Db => StatusCode::INTERNAL_SERVER_ERROR,
            Error::DbReturnedNoRows => StatusCode::NOT_FOUND,
            Error::DbMigrationError => StatusCode::INTERNAL_SERVER_ERROR,
            Error::AxumServerError => StatusCode::INTERNAL_SERVER_ERROR,
            Error::ModelConversionError => StatusCode::INTERNAL_SERVER_ERROR,
            Error::EntityNotFound => StatusCode::NOT_FOUND,
            Error::WrongPassword => StatusCode::UNAUTHORIZED,
            Error::CryptoHashError => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Conflict(_) => StatusCode::CONFLICT,
        };
        (status, Json(self.to_string())).into_response()
    }
}

// impl From<SurrealError> for Error {
//     fn from(error: surrealdb::Error) -> Self {
//         eprintln!("{error}");
//         Self::Db
//     }
// }

// impl From<MongodbError::Error> for Error {
//     fn from(error: MongodbError::Error) -> Self {
//         eprintln!("{error}");
//         Self::Db
//     }
// }

impl From<SqlxError> for Error {
    fn from(error: SqlxError) -> Self {
        eprintln!("{error}");
        match error {
            SqlxError::RowNotFound => Self::DbReturnedNoRows,
            _ => Self::Db,
        }
    }
}
// impl From<QuestDbError> for Error {
//     fn from(error: QuestDbError) -> Self {
//         eprintln!("{error}");
//         Self::Db
//     }
// }
impl<T> From<TrySendError<T>> for Error {
    fn from(error: TrySendError<T>) -> Self {
        eprintln!("{error}");
        Self::InternalServerError
    }
}
// impl From<BsonError> for Error {
//     fn from(error: BsonError) -> Self {
//         eprintln!("{error}");
//         Self::BsonSerializationError
//     }
// }

// }
