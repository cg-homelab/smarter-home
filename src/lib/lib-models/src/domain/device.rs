use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;
use uuid::Uuid;

/// Device metadata persisted for a home.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DomainDevice {
    pub id: Uuid,
    pub home_id: Uuid,
    pub name: String,
    pub device_type: String,
    pub has_power_meter: bool,
    pub is_active: bool,
}

/// Device metadata upsert payload element.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DomainDeviceUpsert {
    pub id: Option<Uuid>,
    pub name: String,
    pub device_type: String,
    pub has_power_meter: bool,
    pub is_active: Option<bool>,
}

/// Request payload for upserting device metadata in batch for one home.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DomainDevicesIngest {
    pub home_id: Uuid,
    pub devices: Vec<DomainDeviceUpsert>,
}

/// Time-series metric point for a single device.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DomainDeviceMetric {
    pub device_id: Uuid,
    pub ts: DateTime<Utc>,
    pub wattage_w: Option<f64>,
    pub state: Option<String>,
    pub metrics: Option<Value>,
    pub source: Option<String>,
}

/// Request payload for ingesting device metrics for one home.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DomainDeviceMetricsIngest {
    pub home_id: Uuid,
    pub metrics: Vec<DomainDeviceMetric>,
}
