use axum::{extract::State, response::IntoResponse, Json};
use lib_db::home;
use lib_models::domain::home::DomainNewHome;

use crate::routes::AppState;

pub async fn post_home(
    State(state): State<AppState>,
    Json(input): Json<DomainNewHome>,
) -> impl IntoResponse {
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

    let res = home::Home::insert_home(&state.db, &input).await;
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

// pub async fn get_homes() -> impl IntoResponse {
//     let res = store::home::get_homes().await;
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
