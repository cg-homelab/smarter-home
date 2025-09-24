use axum::{extract::Path, response::IntoResponse, Json};
use lib_models::domain::home::NewHome;

use crate::store;

pub async fn post_home(id: Path<String>, Json(input): Json<NewHome>) -> impl IntoResponse {
    let res = store::home::insert_home(&id, input).await;
    match res {
        Ok(item) => {
            tracing::debug!("Metric saved");
            Json(item).into_response()
        }
        Err(error) => {
            tracing::warn!("Metric save failed: {:0}", &error);
            error.into_response()
        }
    }
}

pub async fn get_homes() -> impl IntoResponse {
    let res = store::home::get_homes().await;
    match res {
        Ok(items) => {
            tracing::debug!("Homes retrieved");
            Json(items).into_response()
        }
        Err(error) => {
            tracing::warn!("Homes retrieve failed: {:0}", &error);
            error.into_response()
        }
    }
}
