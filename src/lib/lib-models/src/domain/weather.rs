use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;
use uuid::Uuid;

/// Weather observation point for a home at a specific timestamp.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DomainWeatherObservation {
    pub ts: DateTime<Utc>,
    pub provider: Option<String>,
    pub temp_c: Option<f64>,
    pub clouds_pct: Option<f64>,
    pub rain_mm_1h: Option<f64>,
    pub wind_ms: Option<f64>,
    pub pressure_hpa: Option<f64>,
    pub humidity_pct: Option<f64>,
    pub uvi: Option<f64>,
    pub solar_irradiance_wm2: Option<f64>,
    pub weather_code: Option<String>,
    pub raw: Option<Value>,
}

/// Request payload for weather observation ingest for one home.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DomainWeatherObservationsIngest {
    pub home_id: Uuid,
    pub observations: Vec<DomainWeatherObservation>,
}

/// Forecast point for a specific model run and valid timestamp.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DomainWeatherForecast {
    pub run_ts: DateTime<Utc>,
    pub valid_ts: DateTime<Utc>,
    pub horizon_hours: i32,
    pub provider: Option<String>,
    pub temp_c: Option<f64>,
    pub clouds_pct: Option<f64>,
    pub rain_mm: Option<f64>,
    pub wind_ms: Option<f64>,
    pub uvi: Option<f64>,
    pub raw: Option<Value>,
}

/// Request payload for weather forecast ingest for one home.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DomainWeatherForecastsIngest {
    pub home_id: Uuid,
    pub forecasts: Vec<DomainWeatherForecast>,
}
