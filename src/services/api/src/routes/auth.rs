use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use chrono::{Duration, Utc};
use lib_models::domain::auth::{AuthBody, LogoutRequest, RefreshTokenRequest};
use lib_models::domain::user::{AuthUser, DomainUser, NewDomainUser};
use lib_models::{error::Error, Role};
use lib_utils::crypto;
use serde_json::json;

use super::AppState;

async fn issue_auth_body(
    state: &AppState,
    user: &DomainUser,
    message: String,
) -> Result<AuthBody, Error> {
    let access_token = crypto::generate_jwt(
        user.email.clone(),
        Role::from_text(user.role.as_str()),
        Some(user.id),
        false,
    );
    let refresh_token = crypto::generate_refresh_token();
    let refresh_token_hash = crypto::hash_refresh_token(&refresh_token);
    let refresh_expires_at = Utc::now() + Duration::seconds(crypto::refresh_token_ttl_secs());

    lib_db::refresh_token::RefreshToken::create(
        &state.db,
        user.id,
        &refresh_token_hash,
        refresh_expires_at,
    )
    .await?;

    Ok(AuthBody::new(
        access_token,
        refresh_token,
        crypto::access_token_ttl_secs(),
        message,
    ))
}

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
        Ok(valid_user) => match issue_auth_body(
            &state,
            &valid_user,
            "User signed in successfully".to_string(),
        )
        .await
        {
            Ok(response) => Ok(Json(response).into_response()),
            Err(e) => Err(e.into_response()),
        },
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
            match issue_auth_body(&state, &user, "User signed up successfully".to_string()).await {
                Ok(response) => Ok(Json(response).into_response()),
                Err(e) => Err(e.into_response()),
            }
        }
    }
}

/// Rotates an active refresh token and returns a new token pair.
#[utoipa::path(
    post,
    path = "/auth/refresh",
    tag = "auth",
    request_body = RefreshTokenRequest,
    responses(
        (status = 200, description = "Token refreshed successfully", body = AuthBody),
        (status = 401, description = "Unauthorized", body = String),
        (status = 500, description = "Internal Server Error", body = String),
    )
)]
pub async fn refresh_token(
    State(state): State<AppState>,
    Json(request): Json<RefreshTokenRequest>,
) -> impl IntoResponse {
    let token_hash = crypto::hash_refresh_token(&request.refresh_token);

    let current =
        match lib_db::refresh_token::RefreshToken::get_active_by_hash(&state.db, &token_hash).await
        {
            Ok(token) => token,
            Err(e) => return Err(e.into_response()),
        };

    let user = match lib_db::user::User::get(&state.db, current.user_id).await {
        Ok(user) => user,
        Err(e) => return Err(e.into_response()),
    };

    let new_refresh_token = crypto::generate_refresh_token();
    let new_refresh_hash = crypto::hash_refresh_token(&new_refresh_token);
    let refresh_expires_at = Utc::now() + Duration::seconds(crypto::refresh_token_ttl_secs());

    if let Err(e) = lib_db::refresh_token::RefreshToken::rotate(
        &state.db,
        current.id,
        &new_refresh_hash,
        refresh_expires_at,
    )
    .await
    {
        return Err(e.into_response());
    }

    let access_token = crypto::generate_jwt(
        user.email,
        Role::from_text(user.role.as_str()),
        Some(user.id),
        false,
    );

    Ok(Json(AuthBody::new(
        access_token,
        new_refresh_token,
        crypto::access_token_ttl_secs(),
        "Token refreshed successfully".to_string(),
    ))
    .into_response())
}

/// Revokes a refresh token.
#[utoipa::path(
    post,
    path = "/auth/logout",
    tag = "auth",
    request_body = LogoutRequest,
    responses(
        (status = 200, description = "Logged out successfully", body = String),
        (status = 500, description = "Internal Server Error", body = String),
    )
)]
pub async fn log_out(
    State(state): State<AppState>,
    Json(request): Json<LogoutRequest>,
) -> impl IntoResponse {
    let token_hash = crypto::hash_refresh_token(&request.refresh_token);

    match lib_db::refresh_token::RefreshToken::revoke_by_hash(&state.db, &token_hash).await {
        Ok(_) => Ok(Json(json!({ "message": "Logged out successfully" })).into_response()),
        Err(e) => Err(e.into_response()),
    }
}
