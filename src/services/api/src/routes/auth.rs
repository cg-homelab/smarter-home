use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::Json;
use lib_models::domain::user::{AuthUser, NewDomainUser};
use lib_utils::crypto;

use super::AppState;

pub async fn jwt_auth(
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let headers = req.headers();
    if let Some(auth_header) = headers.get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                match crypto::validate_jwt(state.encoding_key.as_bytes(), token) {
                    Ok(_) => return Ok(next.run(req).await), // JWT is valid, proceed to next handler
                    Err(_) => return Err(StatusCode::UNAUTHORIZED), // Invalid JWT
                }
            }
        }
    }
    Err(StatusCode::UNAUTHORIZED) // No Authorization header or invalid token
}

pub async fn log_in(
    State(state): State<AppState>,
    Json(auth_user): Json<AuthUser>,
) -> impl IntoResponse {
    let password_valid = lib_db::user::User::auth_user(&state.db, &auth_user).await;
    match password_valid {
        Err(e) => {
            tracing::error!("Error authenticating user: {:?}", e);
            let response = serde_json::json!({
                "message": "Internal server error"
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        }
        Ok(valid) => {
            if valid {
                let token = crypto::generate_jwt(state.encoding_key.as_bytes(), auth_user.email);
                let response = serde_json::json!({
                    "message": "User signed in successfully",
                    "token": token,
                });
                (StatusCode::OK, Json(response))
            } else {
                let response = serde_json::json!({
                    "message": "Invalid email or password"
                });
                (StatusCode::UNAUTHORIZED, Json(response))
            }
        }
    }
}

pub async fn sign_up(
    State(state): State<AppState>,
    Json(new_user): Json<NewDomainUser>,
) -> impl IntoResponse {
    // Here you would typically add code to save the new user to your database
    //
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

            let token = crypto::generate_jwt(state.encoding_key.as_bytes(), user.email.clone());
            let response = serde_json::json!({
                "message": "User signed up successfully",
                "token": token,
            });
            (StatusCode::CREATED, Json(response))
        }
    }
}
