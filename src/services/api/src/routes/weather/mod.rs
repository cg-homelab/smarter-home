use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use lib_db::weather::{WeatherForecast, WeatherObservation};
use lib_models::{
    domain::weather::{DomainWeatherForecastsIngest, DomainWeatherObservationsIngest},
    error::Error,
};
use lib_utils::crypto::Claims;
use reqwest::StatusCode;

use crate::routes::AppState;

#[utoipa::path(
    post,
    path = "/weather/observations",
    tag = "weather",
    request_body = DomainWeatherObservationsIngest,
    responses(
        (status = 200, description = "Weather observations ingested", body = serde_json::Value),
        (status = 401, description = "Unauthorized", body = serde_json::Value),
        (status = 500, description = "Internal Server Error", body = serde_json::Value),
    )
)]
pub async fn post_weather_observations(
    claims: Claims,
    State(state): State<AppState>,
    Json(input): Json<DomainWeatherObservationsIngest>,
) -> impl IntoResponse {
    if let Some(home_id) = claims.id {
        if home_id != input.home_id {
            let response = serde_json::json!({
                "message": "Unauthorized",
            });
            return (StatusCode::UNAUTHORIZED, Json(response)).into_response();
        }

        match WeatherObservation::insert_observations(&state.db, &input).await {
            Ok(rows) => {
                let response = serde_json::json!({
                    "message": "Weather observations ingested",
                    "rowsAffected": rows,
                });
                (StatusCode::OK, Json(response)).into_response()
            }
            Err(error) => {
                tracing::warn!("Weather observation ingest failed: {}", error);
                error.into_response()
            }
        }
    } else {
        let error = Error::Unauthorized;
        tracing::warn!("Weather observation ingest failed: {}", error);
        let response = serde_json::json!({
            "message": "Unauthorized",
        });
        (StatusCode::UNAUTHORIZED, Json(response)).into_response()
    }
}

#[utoipa::path(
    post,
    path = "/weather/forecasts",
    tag = "weather",
    request_body = DomainWeatherForecastsIngest,
    responses(
        (status = 200, description = "Weather forecasts ingested", body = serde_json::Value),
        (status = 401, description = "Unauthorized", body = serde_json::Value),
        (status = 500, description = "Internal Server Error", body = serde_json::Value),
    )
)]
pub async fn post_weather_forecasts(
    claims: Claims,
    State(state): State<AppState>,
    Json(input): Json<DomainWeatherForecastsIngest>,
) -> impl IntoResponse {
    if let Some(home_id) = claims.id {
        if home_id != input.home_id {
            let response = serde_json::json!({
                "message": "Unauthorized",
            });
            return (StatusCode::UNAUTHORIZED, Json(response)).into_response();
        }

        match WeatherForecast::insert_forecasts(&state.db, &input).await {
            Ok(rows) => {
                let response = serde_json::json!({
                    "message": "Weather forecasts ingested",
                    "rowsAffected": rows,
                });
                (StatusCode::OK, Json(response)).into_response()
            }
            Err(error) => {
                tracing::warn!("Weather forecast ingest failed: {}", error);
                error.into_response()
            }
        }
    } else {
        let error = Error::Unauthorized;
        tracing::warn!("Weather forecast ingest failed: {}", error);
        let response = serde_json::json!({
            "message": "Unauthorized",
        });
        (StatusCode::UNAUTHORIZED, Json(response)).into_response()
    }
}
