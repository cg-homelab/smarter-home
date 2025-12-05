use chrono::{DateTime, Utc};
use lib_models::{domain::power::PowerMetrics as DomainPowerMetric, error::Error};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{Db, home::Home};

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
    /// Insert Power Metric into database
    /// # Arguments
    /// * `db` - Database connection
    /// * `new_powermetric` - Power Metric data
    /// # Returns
    /// * `Result<bool, Error>` - true or error
    pub async fn insert(db: &Db, new_powermetric: &DomainPowerMetric) -> Result<bool, Error> {
        let _row = sqlx::query!(
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
        .execute(&db.pool)
        .await?;

        Ok(true)
    }

    /// Get powermetrics for home user has access to between two timestamps
    /// # Arguments
    /// * `db` - Database connection
    /// * `home_id` - Home ID
    /// * `user_id` - User ID
    /// * `start_ts` - Start timestamp
    /// * `end_ts` - End timestamp
    /// # Returns
    /// * `Result<Vec<DomainPowerMetric>, Error>` - Vector of Power Metrics or error
    pub async fn get_power_metrics(
        db: &Db,
        home_id: Uuid,
        user_id: Uuid,
        start_ts: DateTime<Utc>,
        end_ts: DateTime<Utc>,
    ) -> Result<Vec<DomainPowerMetric>, Error> {
        // check if user has access to home
        let user_in_home = Home::check_user_on_home(db, home_id, user_id).await?;
        if !user_in_home {
            return Err(Error::Unauthorized);
        }

        let power_metrics = sqlx::query_as!(
            PowerMetric,
            r#"
            SELECT
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
            currency
            FROM power_metrics
            WHERE home_id = $1 AND ts >= $2 AND ts <= $3
            ORDER BY ts ASC
            "#,
            home_id,
            start_ts,
            end_ts,
        )
        .fetch_all(&db.pool)
        .await?
        .iter()
        .map(|pm| DomainPowerMetric {
            home_id: pm.home_id,
            ts: pm.ts,
            price: pm.price,
            power: pm.power,
            solar_power: pm.solar_power,
            last_meter_consumption: pm.last_meter_consumption,
            last_meter_production: pm.last_meter_production,
            last_solar_total: pm.last_solar_total,
            consumption_since_midnight: pm.consumption_since_midnight,
            production_since_midnight: pm.production_since_midnight,
            solar_since_midnight: pm.solar_since_midnight,
            cost_since_midnight: pm.cost_since_midnight,
            currency: pm.currency.clone(),
        })
        .collect::<Vec<DomainPowerMetric>>();

        Ok(power_metrics)
    }
}
