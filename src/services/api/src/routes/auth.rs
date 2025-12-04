use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use lib_models::domain::user::{AuthUser, NewDomainUser};
use lib_models::Role;
use lib_utils::crypto;

use super::AppState;

#[utoipa::path(
    post,
    path = "/user/login",
    tag = "auth",
    request_body = AuthUser,
    responses(
        (status = 200, description = "User logged in successfully", body = serde_json::Value),
        (status = 500, description = "Internal Server Error", body = serde_json::Value),
    )
)]
pub async fn log_in(
    State(state): State<AppState>,
    Json(auth_user): Json<AuthUser>,
) -> impl IntoResponse {
    let user_result = lib_db::user::User::auth_user(&state.db, &auth_user).await;
    match user_result {
        Err(e) => {
            tracing::error!("Error authenticating user: {:?}", e);
            let response = serde_json::json!({
                "message": "Internal server error"
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        }
        Ok(valid_user) => {
            let token = crypto::generate_jwt(
                valid_user.email,
                Role::from_text(valid_user.role.as_str()),
                Some(valid_user.id),
                false,
            );
            let response = serde_json::json!({
                "message": "User signed in successfully",
                "token": token,
            });
            (StatusCode::OK, Json(response))
        }
    }
}

#[utoipa::path(
    post,
    path = "/user/signup",
    tag = "auth",
    request_body = NewDomainUser,
    responses(
        (status = 201, description = "User signed up successfully", body = serde_json::Value),
        (status = 409, description = "Conflict - User already exists", body = serde_json::Value),
        (status = 500, description = "Internal Server Error", body = serde_json::Value),
    )
)]
pub async fn sign_up(
    State(state): State<AppState>,
    Json(new_user): Json<NewDomainUser>,
) -> impl IntoResponse {
    let user_exists = lib_db::user::User::check_exists(&state.db, &new_user).await;

    if let Ok(exists) = user_exists {
        if exists {
            let response = serde_json::json!({
                "message": "User with this email already exists"
            });
            return (StatusCode::CONFLICT, Json(response));
        }
    }

    let created_user = lib_db::user::User::insert(&state.db, &new_user).await;

    match created_user {
        Err(e) => {
            tracing::error!("Error creating user: {:?}", e);
            let response = serde_json::json!({
                "message": "Internal server error"
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        }
        Ok(user) => {
            tracing::info!("User created successfully: {:?}", user.email);

            let token = crypto::generate_jwt(user.email.clone(), Role::User, Some(user.id), false);
            let response = serde_json::json!({
                "message": "User signed up successfully",
                "token": token,
            });
            (StatusCode::CREATED, Json(response))
        }
    }
}
