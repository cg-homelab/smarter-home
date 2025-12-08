use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use lib_models::domain::auth::AuthBody;
use lib_models::domain::user::{AuthUser, NewDomainUser};
use lib_models::{error::Error, Role};
use lib_utils::crypto;

use super::AppState;

/// Authenticates a user and returns a JWT token upon successful login.
#[utoipa::path(
    post,
    path = "/auth/login",
    tag = "auth",
    request_body = AuthUser,
    responses(
        (status = 200, description = "User logged in successfully", body = AuthBody),
        (status = 500, description = "Internal Server Error", body = String),
    )
)]
pub async fn log_in(
    State(state): State<AppState>,
    Json(auth_user): Json<AuthUser>,
) -> impl IntoResponse {
    let user_result = lib_db::user::User::auth_user(&state.db, &auth_user).await;
    match user_result {
        Err(e) => {
            tracing::debug!("Error authenticating user: {:?}", e);
            Err(e.into_response())
        }
        Ok(valid_user) => {
            let token = crypto::generate_jwt(
                valid_user.email,
                Role::from_text(valid_user.role.as_str()),
                Some(valid_user.id),
                false,
            );
            let response = AuthBody::new(token, "User signed in successfully".to_string());
            Ok(Json(response).into_response())
        }
    }
}

/// Registers a new user and returns a JWT token upon successful signup.
#[utoipa::path(
    post,
    path = "/auth/signup",
    tag = "auth",
    request_body = NewDomainUser,
    responses(
        (status = 200, description = "User signed up successfully", body = AuthBody),
        (status = 409, description = "Conflict - User already exists", body = String),
        (status = 500, description = "Internal Server Error", body = String),
    )
)]
pub async fn sign_up(
    State(state): State<AppState>,
    Json(new_user): Json<NewDomainUser>,
) -> impl IntoResponse {
    let user_exists = lib_db::user::User::check_exists(&state.db, &new_user).await;

    if let Ok(exists) = user_exists {
        if exists {
            return Err(
                Error::Conflict("User with this email already exists".to_string()).into_response(),
            );
        }
    }

    let created_user = lib_db::user::User::insert(&state.db, &new_user).await;

    match created_user {
        Err(e) => Err(e.into_response()),
        Ok(user) => {
            tracing::debug!("User created successfully: {:?}", user.email);

            let token = crypto::generate_jwt(user.email.clone(), Role::User, Some(user.id), false);
            Ok(Json(AuthBody::new(
                token,
                "User signed up successfully".to_string(),
            ))
            .into_response())
        }
    }
}
