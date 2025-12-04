use chrono::{DateTime, Utc};
use lib_models::{domain::power::PowerMetrics as DomainPowerMetric, error::Error};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::Db;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerMetric {
    pub home_id: Uuid,
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
impl PowerMetric {
    pub async fn insert(db: &Db, new_powermetric: &DomainPowerMetric) -> Result<bool, Error> {
        let user = sqlx::query!(
            r#"
            INSERT INTO power_metrics (
                home_id,
                ts,
                price,
                power,
                solar_power,
                last_meter_consumption,
                last_meter_production,
                last_solar_total,
                consumption_since_midnight,
                production_since_midnight,
                solar_since_midnight,
                cost_since_midnight,
                currency)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            "#,
            new_powermetric.home_id,
            new_powermetric.ts,
            new_powermetric.price,
            new_powermetric.power,
            new_powermetric.solar_power,
            new_powermetric.last_meter_consumption,
            new_powermetric.last_meter_production,
            new_powermetric.last_solar_total,
            new_powermetric.consumption_since_midnight,
            new_powermetric.production_since_midnight,
            new_powermetric.solar_since_midnight,
            new_powermetric.cost_since_midnight,
            new_powermetric.currency,
        )
        .fetch_one(&db.pool)
        .await?;

        Ok(true)
    }
}
