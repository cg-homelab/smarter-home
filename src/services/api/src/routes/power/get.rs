use axum::extract::{Query, State};
use axum::response::IntoResponse;
use axum::Json;
use chrono::{DateTime, Utc};
use lib_db::power::PowerMetric;
use lib_models::error::Error;
use lib_utils::crypto::Claims;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::routes::AppState;

/// Query parameters for fetching power metrics within a specified period.
#[derive(Deserialize, Serialize, ToSchema)]
pub struct PowerMetricsQuery {
    pub home_id: Uuid,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
}

/// Retrieves power metrics for a specified home within a given date range.
#[utoipa::path(
    get,
    path = "/power/metrics",
    tag = "power",
    params(
        ("home_id" = Uuid, Query, description = "The ID of the home"),
        ("start_date" = DateTime<Utc>, Query, description = "The start date for the metrics range"),
        ("end_date" = DateTime<Utc>, Query, description = "The end date for the metrics range"),
    ),
    responses(
        (status = 200, description = "Fetched power metrics successfully", body = Vec<lib_models::domain::power::PowerMetrics>),
        (status = 401, description = "Unauthorized", body = String),
        (status = 500, description = "Internal Server Error", body = String),
    )
)]
pub async fn get_metrics_for_period(
    claims: Claims,
    State(state): State<AppState>,
    Query(params): Query<PowerMetricsQuery>,
) -> impl IntoResponse {
    let user_id = match claims.id {
        Some(id) => id,
        None => {
            tracing::warn!("Unauthorized access attempt: no user ID in claims");
            let error = Error::Unauthorized;
            return Err(error.into_response());
        }
    };

    let metrics_result = PowerMetric::get_power_metrics(
        &state.db,
        params.home_id,
        user_id,
        params.start_date,
        params.end_date,
    )
    .await;

    match metrics_result {
        Ok(metrics) => Ok(Json(metrics).into_response()),
        Err(e) => {
            tracing::error!("Error fetching power metrics: {:?}", e);
            Err(e.into_response())
        }
    }
}
