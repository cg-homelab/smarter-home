use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::Json;
use lib_models::domain::user::NewUser;
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

pub async fn sign_up(Json(new_user): Json<NewUser>) -> impl IntoResponse {
    // Here you would typically add code to save the new user to your database
    // For this example, we'll just return a success message
    // Generate a JWT for the new user
    let token = crypto::generate_jwt(
        std::env::var("AUTH_SECRET").unwrap().as_bytes(),
        new_user.email.clone(),
    );
    let response = serde_json::json!({
        "message": "User signed up successfully",
        "token": token,
    });
    (StatusCode::CREATED, Json(response))
}
