use axum::{http::StatusCode, response::IntoResponse, response::Response};

use crate::error::ApiError;

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let response = match self {
            Self::SQL(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            Self::Request(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            Self::TokenError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            Self::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized!".to_string()),
            Self::OAuthError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error in OAuth!".to_string(),
            ),
            Self::OptionError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Attempted to get a non-none value but found none".to_string(),
            ),
            Self::ParseIntError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            Self::FromRequestPartsError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            Self::NotImplemented => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Not implemented".to_string(),
            ),
        };

        response.into_response()
    }
}
