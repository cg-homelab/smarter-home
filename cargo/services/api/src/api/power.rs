use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use lib_database::power::write_power_metric;
use lib_models::domain::power::PowerMetrics;
use sqlx::PgPool;

pub async fn post_power_metric(
    State(pool): State<PgPool>,
    Json(input): Json<PowerMetrics>,
) -> impl IntoResponse {
    // TODO write to db
    let res = write_power_metric(&pool, input).await;
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
