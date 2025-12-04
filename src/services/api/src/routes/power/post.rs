use axum::extract::State;
// use crate::routes::AppState;
use axum::response::IntoResponse;
use axum::Json;
//{extract::State, response::IntoResponse, Json}
//use lib_models::domain::power::PowerMetrics;
// use lib_models::entity::timeserie::TimeSerieRecord;
use lib_db::power::PowerMetric as DbPowerMetric;
use lib_models::domain::power::PowerMetrics;
use lib_models::error::Error;
use reqwest::StatusCode;

use crate::routes::AppState;

#[utoipa::path(
    post,
    path = "/power/metric",
    tag = "power",
    request_body = PowerMetrics,
    responses(
        (status = 200, description = "Power metric saved successfully", body = serde_json::Value),
        (status = 401, description = "Unauthorized", body = serde_json::Value),
        (status = 500, description = "Internal Server Error", body = serde_json::Value),
    )
)]
pub async fn post_power_metric(
    claims: lib_utils::crypto::Claims,
    State(state): State<AppState>,
    Json(input): Json<PowerMetrics>,
) -> impl IntoResponse {
    if let Some(user_id) = claims.id {
        tracing::debug!(
            "User {} is attempting to save a new power metric for home id {}",
            user_id,
            &input.home_id
        );
        if user_id != input.home_id {
            tracing::warn!("Sender not authorized to post metric for this home_id");
            let response = serde_json::json!({
                "message": "Unauthorized",
            });
            return (StatusCode::UNAUTHORIZED, Json(response));
        }

        let result = DbPowerMetric::insert(&state.db, &input)
            .await
            .inspect_err(|error| {
                tracing::warn!("Power metric save failed: {:0}", &error);
            });

        match result {
            Err(_) => {
                let response = serde_json::json!({
                    "message": "Internal server error",
                });
                (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
            }
            Ok(_) => {
                let response = serde_json::json!({
                    "message": "Metric saved successfully",
                });
                (StatusCode::OK, Json(response))
            }
        }
    } else {
        let error = Error::Unauthorized;
        tracing::warn!("Power metric save failed: {:0}", &error);
        let response = serde_json::json!({
            "message": "Unauthorized",
        });
        (StatusCode::UNAUTHORIZED, Json(response))
    }
}
