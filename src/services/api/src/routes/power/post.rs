use crate::store::power::write_power_metric;
use axum::{response::IntoResponse, Json};
use lib_models::domain::power::PowerMetrics;

pub async fn post_power_metric(Json(input): Json<PowerMetrics>) -> impl IntoResponse {
    let res = write_power_metric(input).await;
    match res {
        Ok(item) => {
            tracing::debug!("Metric saved");
            Json(PowerMetrics::from_entity(item)).into_response()
        }
        Err(error) => {
            tracing::warn!("Metric save failed: {:0}", &error);
            error.into_response()
        }
    }
}
