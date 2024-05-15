use crate::{routes::AppState, store::power::write_power_metric};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use lib_models::domain::power::PowerMetrics;

pub async fn post_power_metric(
    State(state): State<AppState>,
    Json(input): Json<PowerMetrics>,
) -> impl IntoResponse {
    // TODO write to db
    let res = write_power_metric(&state.db, input).await;
    match res {
        Ok(_) => {
            tracing::debug!("Metric saved");
        }
        Err(val) => {
            tracing::warn!("Metric save failed: {:0}", val);
        }
    }

    StatusCode::CREATED
}
