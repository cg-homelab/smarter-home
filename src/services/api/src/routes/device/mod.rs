use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use lib_db::device::{Device as DbDevice, DeviceMetric as DbDeviceMetric};
use lib_models::{
    domain::device::{DomainDevice, DomainDeviceMetricsIngest, DomainDevicesIngest},
    error::Error,
};
use lib_utils::crypto::Claims;

use crate::routes::AppState;

#[utoipa::path(
    post,
    path = "/devices",
    tag = "device",
    request_body = DomainDevicesIngest,
    responses(
        (status = 200, description = "Device metadata ingested", body = Vec<DomainDevice>),
        (status = 401, description = "Unauthorized", body = serde_json::Value),
        (status = 500, description = "Internal Server Error", body = serde_json::Value),
    )
)]
pub async fn post_devices(
    claims: Claims,
    State(state): State<AppState>,
    Json(input): Json<DomainDevicesIngest>,
) -> impl IntoResponse {
    if let Some(home_id) = claims.id {
        if home_id != input.home_id {
            let response = serde_json::json!({
                "message": "Unauthorized",
            });
            return (StatusCode::UNAUTHORIZED, Json(response)).into_response();
        }

        match DbDevice::upsert_devices(&state.db, &input).await {
            Ok(devices) => (StatusCode::OK, Json(devices)).into_response(),
            Err(error) => {
                tracing::warn!("Device upsert failed: {}", error);
                error.into_response()
            }
        }
    } else {
        let error = Error::Unauthorized;
        tracing::warn!("Device upsert failed: {}", error);
        let response = serde_json::json!({
            "message": "Unauthorized",
        });
        (StatusCode::UNAUTHORIZED, Json(response)).into_response()
    }
}

#[utoipa::path(
    post,
    path = "/devices/metrics",
    tag = "device",
    request_body = DomainDeviceMetricsIngest,
    responses(
        (status = 200, description = "Device metrics ingested", body = serde_json::Value),
        (status = 401, description = "Unauthorized", body = serde_json::Value),
        (status = 500, description = "Internal Server Error", body = serde_json::Value),
    )
)]
pub async fn post_device_metrics(
    claims: Claims,
    State(state): State<AppState>,
    Json(input): Json<DomainDeviceMetricsIngest>,
) -> impl IntoResponse {
    if let Some(home_id) = claims.id {
        if home_id != input.home_id {
            let response = serde_json::json!({
                "message": "Unauthorized",
            });
            return (StatusCode::UNAUTHORIZED, Json(response)).into_response();
        }

        match DbDeviceMetric::insert_metrics(&state.db, &input).await {
            Ok(rows) => {
                let response = serde_json::json!({
                    "message": "Device metrics ingested",
                    "rowsAffected": rows,
                });
                (StatusCode::OK, Json(response)).into_response()
            }
            Err(error) => {
                tracing::warn!("Device metrics ingest failed: {}", error);
                error.into_response()
            }
        }
    } else {
        let error = Error::Unauthorized;
        tracing::warn!("Device metrics ingest failed: {}", error);
        let response = serde_json::json!({
            "message": "Unauthorized",
        });
        (StatusCode::UNAUTHORIZED, Json(response)).into_response()
    }
}
