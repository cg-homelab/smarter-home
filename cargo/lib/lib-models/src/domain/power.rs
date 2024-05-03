use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerMetrics {
    pub home_id: uuid::Uuid,
    pub ts: DateTime<Utc>,
    pub power: f64,
    pub min_power: f64,
    pub max_power: f64,
    pub average_power: f64,
    pub last_meter_consumption: f64,
    pub last_meter_production: f64,
    pub accumulated_consumption: f64,
    pub accumulated_production: f64,
    pub accumulated_cost: f64,
    pub accumulated_consumption_last_hour: f64,
    pub accumulated_production_last_hour: f64,
    pub currency: String,
}
