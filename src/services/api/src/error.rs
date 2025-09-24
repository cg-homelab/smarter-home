// use thiserror::Error;
// #[derive(Debug, Error)]
// pub enum ApiError {
//     #[error("SQL error: {0}")]
//     SQL(#[from] sqlx::Error),
//     #[error("HTTP request error: {0}")]
//     Request(#[from] reqwest::Error),
//     #[error("OAuth token error: {0}")]
//     TokenError(
//         #[from]
//         oauth2::RequestTokenError<
//             oauth2::reqwest::Error<reqwest::Error>,
//             oauth2::StandardErrorResponse<oauth2::basic::BasicErrorResponseType>,
//         >,
//     ),
//     #[error("OauthError: {0}")]
//     OAuthError(String),
//     #[error("You're not authorized!")]
//     Unauthorized,
//     #[error("Attempted to get a non-none value but found none")]
//     OptionError,
//     #[error("Attempted to parse a number to an integer but errored out: {0}")]
//     ParseIntError(#[from] std::num::TryFromIntError),
//     #[error("Encountered an error trying to convert an infallible value: {0}")]
//     FromRequestPartsError(#[from] std::convert::Infallible),
//     #[error("This is not implemented yet!")]
//     NotImplemented,
// }

// mod error {
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::Json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("database error")]
    Db,

    #[error("database returned no rows")]
    DbReturnedNoRows,

    #[error("server error")]
    AxumServerError,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(self.to_string())).into_response()
    }
}

impl From<surrealdb::Error> for Error {
    fn from(error: surrealdb::Error) -> Self {
        eprintln!("{error}");
        Self::Db
    }
}
// }
