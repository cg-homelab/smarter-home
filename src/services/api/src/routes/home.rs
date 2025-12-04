use axum::{extract::State, response::IntoResponse, Json};
use lib_db::home;
use lib_models::{domain::home::DomainNewHome, error::Error};

use crate::routes::AppState;

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
    claims: lib_utils::crypto::Claims,
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
                    &input.address
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
            tracing::warn!("Home save failed: {:0}", &error);
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

// pub async fn get_homes(claims: lib_utils::crypto::Claims) -> impl IntoResponse {
//     match
//     let res = home::Home::get_homes().await;
//     match res {
//         Ok(items) => {
//             tracing::debug!("Homes retrieved");
//             Json(items).into_response()
//         }
//         Err(error) => {
//             tracing::warn!("Homes retrieve failed: {:0}", &error);
//             error.into_response()
//         }
//     }
// }
