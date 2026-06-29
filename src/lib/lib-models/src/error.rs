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
use utoipa::ToSchema;

/// Error type returned by spatial hashing helpers.
#[derive(Debug, Error)]
pub enum SpatialHashError {
    /// Latitude/longitude pair could not be converted into a valid H3 coordinate.
    #[error("invalid latitude/longitude: lat={lat}, lng={lng}")]
    InvalidLatLng { lat: f64, lng: f64 },

    /// H3 resolution must be in the range [0, 15].
    #[error("invalid H3 resolution {resolution}; expected 0..=15")]
    InvalidResolution { resolution: u8 },

    /// Provided H3 cell string cannot be parsed.
    #[error("invalid H3 cell index: {cell}")]
    InvalidCell { cell: String },

    /// Provided H3 numeric index cannot be parsed.
    #[error("invalid H3 cell index from u64: {index}")]
    InvalidCellU64 { index: u64 },

    /// Requested parent resolution must be coarser than or equal to the cell resolution.
    #[error(
        "invalid parent resolution {parent_resolution}; cell has resolution {cell_resolution}"
    )]
    InvalidParentResolution {
        parent_resolution: u8,
        cell_resolution: u8,
    },

    /// Requested child resolution must be finer than or equal to the cell resolution.
    #[error("invalid child resolution {child_resolution}; cell has resolution {cell_resolution}")]
    InvalidChildResolution {
        child_resolution: u8,
        cell_resolution: u8,
    },

    /// Grid distance failed due to H3 constraints (for example pentagon distortion or mismatch).
    #[error("failed to compute grid distance: {details}")]
    GridDistance { details: String },

    /// GeoJSON payload could not be parsed.
    #[error("invalid GeoJSON payload: {details}")]
    InvalidGeoJson { details: String },

    /// GeoJSON geometry type is unsupported by polygon coverage.
    #[error("unsupported GeoJSON geometry type: {geometry_type}")]
    UnsupportedGeoJsonGeometry { geometry_type: String },

    /// GeoJSON ring coordinate is invalid.
    #[error("invalid GeoJSON coordinate at index {index}: {details}")]
    InvalidGeoJsonCoordinate { index: usize, details: String },

    /// GeoJSON contains no polygon/multipolygon geometry to cover.
    #[error("GeoJSON does not contain polygon geometries")]
    MissingPolygonGeometry,

    /// Polygon geometry is invalid for H3 tiling.
    #[error("invalid polygon geometry for tiling: {details}")]
    InvalidGeometry { details: String },

    /// Async batch worker could not be joined.
    #[error("failed to join async batch task: {details}")]
    AsyncBatchJoin { details: String },
}

/// Custom error type for the application
/// # Variants
/// * `InternalServerError` - Internal server error
/// * `Db` - Database error
/// * `DbReturnedNoRows` - Database returned no rows
/// * `DbMigrationError` - Database migration error
/// * `AxumServerError` - Axum server error
/// * `ModelConversionError` - Model conversion error
/// * `EntityNotFound` - Entity not found
/// * `WrongPassword` - Wrong password
/// * `Unauthorized` - Unauthorized action
/// * `CryptoHashError` - Crypto hash error
/// * `Conflict` - Conflict error with message
/// * `InvalidToken` - Invalid bearer token
/// * `SpatialHash` - Spatial hashing error with details
#[derive(Error, Debug, ToSchema)]
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

    #[error("Unotherized to do this action")]
    Unauthorized,

    #[error("crypto hash error")]
    CryptoHashError,

    #[error("conflict: {0}")]
    Conflict(String),

    #[error("Invalid Bearer token")]
    InvalidToken,

    #[error("Forbidden")]
    Forbidden,

    #[error("spatial hash error: {0}")]
    SpatialHash(String),
}
/// Implement IntoResponse for Error to convert it into an HTTP response
impl IntoResponse for Error {
    /// Convert the Error into an HTTP response
    /// # Returns
    /// * `Response` - HTTP response with status code and error message
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
            Error::Unauthorized => StatusCode::UNAUTHORIZED,
            Error::Conflict(_) => StatusCode::CONFLICT,
            Error::InvalidToken => StatusCode::UNAUTHORIZED,
            Error::Forbidden => StatusCode::FORBIDDEN,
            Error::SpatialHash(_) => StatusCode::BAD_REQUEST,
        };
        (status, Json(self.to_string())).into_response()
    }
}

impl From<SpatialHashError> for Error {
    /// Convert a SpatialHashError into the shared application Error type.
    fn from(error: SpatialHashError) -> Self {
        Self::SpatialHash(error.to_string())
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
    /// Convert a SqlxError into a custom Error
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
    /// Convert a TrySendError into a custom Error
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

#[cfg(test)]
mod tests {
    use super::{Error, SpatialHashError};

    #[test]
    fn spatial_hash_error_converts_into_error() {
        let source = SpatialHashError::InvalidResolution { resolution: 99 };
        let converted: Error = source.into();

        match converted {
            Error::SpatialHash(message) => {
                assert!(message.contains("invalid H3 resolution"));
            }
            other => panic!("unexpected converted error variant: {other}"),
        }
    }
}
