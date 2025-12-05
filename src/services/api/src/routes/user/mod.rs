use axum::{extract::State, response::IntoResponse, Json};
use lib_db::user::User;
use lib_models::error::Error;
use lib_utils::crypto::Claims;

use crate::routes::AppState;

/// Retrieves information about the authenticated user.
#[utoipa::path(
    get,
    path = "/user/me",
    tag = "user",
    responses(
        (status = 200, description = "Fetched user info successfully", body = lib_models::domain::user::DomainUser),
        (status = 401, description = "Unauthorized", body = String),
        (status = 500, description = "Internal Server Error", body = String),
    )
)]
pub async fn get_me(claims: Claims, State(state): State<AppState>) -> impl IntoResponse {
    let id = match claims.id {
        Some(user_id) => user_id,
        None => {
            tracing::warn!("Unauthorized access attempt: no user ID in claims");
            let error = Error::Unauthorized;
            return Err(error.into_response());
        }
    };
    let user = User::get(&state.db, id).await;

    match user {
        Ok(user) => Ok(Json(user).into_response()),
        Err(e) => {
            tracing::error!("Error fetching user: {:?}", e);
            Err(e.into_response())
        }
    }
}

