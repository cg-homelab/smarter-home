use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PowerMetrics {
    pub home_id: String,
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
impl PowerMetrics {
    pub fn from_domain(model: crate::domain::power::PowerMetrics) -> Self {
        Self {
            home_id: model.home_id,
            ts: model.ts,
            price: model.price,
            power: model.power,
            solar_power: model.solar_power,
            last_meter_consumption: model.last_meter_consumption,
            last_meter_production: model.last_meter_production,
            last_solar_total: model.last_solar_total,
            consumption_since_midnight: model.consumption_since_midnight,
            production_since_midnight: model.production_since_midnight,
            solar_since_midnight: model.solar_since_midnight,
            cost_since_midnight: model.cost_since_midnight,
            currency: model.currency,
        }
    }
}
