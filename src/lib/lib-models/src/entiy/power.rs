use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PowerMetrics {
    pub home_id: uuid::Uuid,
    pub ts: DateTime<Utc>,
    pub price: f64,       //Current electricity price
    pub power: f64,       //Current wattage draw
    pub solar_power: f64, //Current wattage production
    pub last_meter_consumption: f64,
    pub last_meter_production: f64,
    pub last_solar_total: f64,
    pub consumption_since_midnight: f64,
    pub production_since_midnight: f64,
    pub cost_since_midnight: f64,
    pub currency: String,
}
