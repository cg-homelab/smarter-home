use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use lib_models::domain::{
    auth::AuthBody,
    user::{AuthUser, NewDomainUser},
};
use lib_utils::crypto;

use super::AppState;

// static KEYS: Lazy<crypto::Keys> = Lazy::new(|| {
//     // note that in production, you will probably want to use a random SHA-256 hash or similar
//     let secret = std::env::var("JWT_SECRET").unwrap_or("secret".to_string());
//
//     crypto::Keys::new(secret.as_bytes())
// });

// pub async fn jwt_auth(
//     State(state): State<AppState>,
//     req: Request,
//     next: Next,
// ) -> Result<Response, StatusCode> {
//     let headers = req.headers();
//     if let Some(auth_header) = headers.get("Authorization") {
//         if let Ok(auth_str) = auth_header.to_str() {
//             if let Some(token) = auth_str.strip_prefix("Bearer ") {
//                 match crypto::validate_jwt(state.encoding_key.as_bytes(), token) {
//                     Ok(_) => return Ok(next.run(req).await), // JWT is valid, proceed to next handler
//                     Err(err) => {
//                         tracing::error!("JWT validation error: {:?}", err);
//                         return Err(StatusCode::UNAUTHORIZED);
//                     } // Invalid JWT
//                 }
//             }
//         }
//     }
//     Err(StatusCode::UNAUTHORIZED) // No Authorization header or invalid token
// }

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
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(response)))
        }
        Ok(valid) => {
            if valid {
                let token = crypto::generate_jwt(auth_user.email, false);
                Ok((StatusCode::OK, Json(AuthBody::new(token))))
            } else {
                let response = serde_json::json!({
                    "message": "Invalid email or password"
                });
                Err((StatusCode::UNAUTHORIZED, Json(response)))
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
            return Err((StatusCode::CONFLICT, Json(response)));
        }
    }

    let created_user = lib_db::user::User::insert(&state.db, &new_user).await;

    match created_user {
        Err(e) => {
            tracing::error!("Error creating user: {:?}", e);
            let response = serde_json::json!({
                "message": "Internal server error"
            });
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(response)))
        }
        Ok(user) => {
            tracing::info!("User created successfully: {:?}", user.email);

            let token = crypto::generate_jwt(user.email.clone(), false);
            Ok((StatusCode::CREATED, Json(AuthBody::new(token))))
        }
    }
}
