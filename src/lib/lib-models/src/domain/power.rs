use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

/// PowerMetrics struct representing power metrics in the domain layer
/// # Fields
/// * `home_id` - Home ID
/// * `ts` - Timestamp
/// * `price` - Current electricity price
/// * `power` - Current wattage draw
/// * `solar_power` - Current wattage production
/// * `last_meter_consumption` - Last meter consumption
/// * `last_meter_production` - Last meter production
/// * `last_solar_total` - Last solar total
/// * `consumption_since_midnight` - Consumption since midnight
/// * `production_since_midnight` - Production since midnight
/// * `solar_since_midnight` - Solar since midnight
/// * `cost_since_midnight` - Cost since midnight
/// * `currency` - Currency
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PowerMetrics {
    pub home_id: Uuid, //uuid::Uuid,
    pub ts: DateTime<Utc>,
    pub price: f64,       //Current electricity price
    pub power: f64,       //Current wattage draw
    pub solar_power: f64, //Current wattage production
    pub last_meter_consumption: f64,
    pub last_meter_production: f64,
    pub last_solar_total: f64,
    pub consumption_since_midnight: f64,
    pub production_since_midnight: f64,
    pub solar_since_midnight: f64,
    pub cost_since_midnight: f64,
    pub currency: String,
}
