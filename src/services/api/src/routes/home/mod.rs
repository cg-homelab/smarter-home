use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use lib_db::home;
use lib_models::{
    domain::home::{DomainNewHome, DomainSetFavoriteHome, DomainUpdateHome},
    error::Error,
};
use lib_utils::crypto::Claims;
use uuid::Uuid;

use crate::routes::AppState;

/// Saves a new home associated with the authenticated user.
#[utoipa::path(
    post,
    path = "/home",
    tag = "home",
    request_body = DomainNewHome,
    responses(
        (status = 200, description = "Home saved", body = lib_models::domain::home::DomainHome),
        (status = 401, description = "Unauthorized", body = String),
        (status = 409, description = "Conflict", body = String),
        (status = 500, description = "Internal Server Error", body = String),
    )
)]
pub async fn post_home(
    claims: Claims,
    State(state): State<AppState>,
    Json(input): Json<DomainNewHome>,
) -> impl IntoResponse {
    tracing::debug!(
        "User {} is attempting to save a new home with address {}",
        claims.sub,
        &input.address
    );

    let home_exists = home::Home::check_home_exists(&state.db, &input.address).await;

    match home_exists {
        Ok(exists) => {
            if exists {
                let error = lib_models::error::Error::Conflict(format!(
                    "Home with address {} already exists",
                    input.address
                ));
                tracing::warn!("Home save failed: {:0}", &error);
                return error.into_response();
            }
        }
        Err(error) => {
            tracing::warn!("Home exists check failed: {:0}", &error);
            return error.into_response();
        }
    }

    let user_id = match claims.id {
        Some(user_id) => user_id,
        None => {
            let error = Error::Unauthorized;
            tracing::debug!("Home save failed: {:0}", &error);
            return error.into_response();
        }
    };

    let res = home::Home::insert_home(&state.db, &input, user_id).await;

    match res {
        Ok(item) => {
            tracing::debug!("Home saved");
            Json(item).into_response()
        }
        Err(error) => {
            tracing::warn!("Home save failed: {:0}", &error);
            error.into_response()
        }
    }
}

/// Retrieves all homes associated with the authenticated user.
#[utoipa::path(
    get,
    path = "/home",
    tag = "home",
    responses(
        (status = 200, description = "Fetched homes successfully", body = Vec<lib_models::domain::home::DomainHome>),
        (status = 401, description = "Unauthorized", body = String),
        (status = 500, description = "Internal Server Error", body = String),
    )
    )]
pub async fn get_homes(claims: Claims, State(state): State<AppState>) -> impl IntoResponse {
    let user_id = match claims.id {
        Some(user_id) => user_id,
        None => {
            let error = Error::Unauthorized;
            tracing::debug!("Get homes failed: {:0}", &error);
            return error.into_response();
        }
    };

    let res = home::Home::get_homes(&state.db, user_id).await;

    match res {
        Ok(items) => {
            tracing::debug!("Homes fetched");
            Json(items).into_response()
        }
        Err(error) => {
            tracing::warn!("Get homes failed: {:0}", &error);
            error.into_response()
        }
    }
}

/// Updates the name and address of an existing home owned by the authenticated user.
#[utoipa::path(
    put,
    path = "/home/{id}",
    tag = "home",
    params(
        ("id" = Uuid, Path, description = "Home ID")
    ),
    request_body = DomainUpdateHome,
    responses(
        (status = 200, description = "Home updated", body = lib_models::domain::home::DomainHome),
        (status = 401, description = "Unauthorized", body = String),
        (status = 403, description = "Forbidden", body = String),
        (status = 404, description = "Not found", body = String),
        (status = 500, description = "Internal Server Error", body = String),
    )
)]
pub async fn put_home(
    claims: Claims,
    State(state): State<AppState>,
    Path(home_id): Path<Uuid>,
    Json(input): Json<DomainUpdateHome>,
) -> impl IntoResponse {
    tracing::debug!(
        "User {} is attempting to update home {}",
        claims.sub,
        home_id
    );

    let user_id = match claims.id {
        Some(id) => id,
        None => {
            let error = Error::Unauthorized;
            tracing::debug!("Update home failed: {:0}", &error);
            return error.into_response();
        }
    };

    match home::Home::check_user_on_home(&state.db, home_id, user_id).await {
        Ok(true) => {}
        Ok(false) => {
            let error = Error::Forbidden;
            tracing::warn!("Update home forbidden for user {}", user_id);
            return error.into_response();
        }
        Err(error) => {
            tracing::warn!("Ownership check failed: {:0}", &error);
            return error.into_response();
        }
    }

    match home::Home::update_home(&state.db, home_id, user_id, &input).await {
        Ok(updated) => {
            tracing::debug!("Home {} updated", home_id);
            Json(updated).into_response()
        }
        Err(error) => {
            tracing::warn!("Update home failed: {:0}", &error);
            error.into_response()
        }
    }
}

/// Deletes a home owned by the authenticated user.
#[utoipa::path(
    delete,
    path = "/home/{id}",
    tag = "home",
    params(
        ("id" = Uuid, Path, description = "Home ID")
    ),
    responses(
        (status = 204, description = "Home deleted"),
        (status = 401, description = "Unauthorized", body = String),
        (status = 403, description = "Forbidden", body = String),
        (status = 404, description = "Not found", body = String),
        (status = 500, description = "Internal Server Error", body = String),
    )
)]
pub async fn delete_home(
    claims: Claims,
    State(state): State<AppState>,
    Path(home_id): Path<Uuid>,
) -> impl IntoResponse {
    tracing::debug!(
        "User {} is attempting to delete home {}",
        claims.sub,
        home_id
    );

    let user_id = match claims.id {
        Some(id) => id,
        None => {
            let error = Error::Unauthorized;
            tracing::debug!("Delete home failed: {:0}", &error);
            return error.into_response();
        }
    };

    match home::Home::check_user_on_home(&state.db, home_id, user_id).await {
        Ok(true) => {}
        Ok(false) => {
            let error = Error::Forbidden;
            tracing::warn!("Delete home forbidden for user {}", user_id);
            return error.into_response();
        }
        Err(error) => {
            tracing::warn!("Ownership check failed: {:0}", &error);
            return error.into_response();
        }
    }

    match home::Home::delete_home(&state.db, home_id).await {
        Ok(()) => {
            tracing::debug!("Home {} deleted", home_id);
            axum::http::StatusCode::NO_CONTENT.into_response()
        }
        Err(error) => {
            tracing::warn!("Delete home failed: {:0}", &error);
            error.into_response()
        }
    }
}

/// Sets the favorite status of a home for the authenticated user.
#[utoipa::path(
    patch,
    path = "/home/{id}/favorite",
    tag = "home",
    params(
        ("id" = Uuid, Path, description = "Home ID")
    ),
    request_body = DomainSetFavoriteHome,
    responses(
        (status = 200, description = "Favorite status updated", body = lib_models::domain::home::DomainHome),
        (status = 401, description = "Unauthorized", body = String),
        (status = 403, description = "Forbidden", body = String),
        (status = 404, description = "Not found", body = String),
        (status = 500, description = "Internal Server Error", body = String),
    )
)]
pub async fn patch_home_favorite(
    claims: Claims,
    State(state): State<AppState>,
    Path(home_id): Path<Uuid>,
    Json(input): Json<DomainSetFavoriteHome>,
) -> impl IntoResponse {
    tracing::debug!(
        "User {} is attempting to set favorite={} on home {}",
        claims.sub,
        input.is_favorite,
        home_id
    );

    let user_id = match claims.id {
        Some(id) => id,
        None => {
            let error = Error::Unauthorized;
            tracing::debug!("Set favorite failed: {:0}", &error);
            return error.into_response();
        }
    };

    match home::Home::check_user_on_home(&state.db, home_id, user_id).await {
        Ok(true) => {}
        Ok(false) => {
            let error = Error::Forbidden;
            tracing::warn!("Set favorite forbidden for user {}", user_id);
            return error.into_response();
        }
        Err(error) => {
            tracing::warn!("Ownership check failed: {:0}", &error);
            return error.into_response();
        }
    }

    match home::Home::set_favorite_home(&state.db, home_id, user_id, input.is_favorite).await {
        Ok(updated) => {
            tracing::debug!("Home {} favorite set to {}", home_id, input.is_favorite);
            Json(updated).into_response()
        }
        Err(error) => {
            tracing::warn!("Set favorite failed: {:0}", &error);
            error.into_response()
        }
    }
}
