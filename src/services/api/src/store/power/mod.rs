use lib_models::domain::power::PowerMetrics;
use sqlx::PgPool;

use crate::error::ApiError;

pub async fn write_power_metric(pool: &PgPool, model: PowerMetrics) -> Result<(), ApiError> {
    sqlx::query!(
        // language=PostgrSQL
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
            currency
        )
        VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13);
        "#,
        model.home_id,
        model.ts,
        model.price,
        model.power,
        model.solar_power,
        model.last_meter_consumption,
        model.last_meter_production,
        model.last_solar_total,
        model.consumption_since_midnight,
        model.production_since_midnight,
        model.solar_since_midnight,
        model.cost_since_midnight,
        model.currency
    )
    .execute(pool)
    .await?;
    Ok(())
}
