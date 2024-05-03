use lib_models::{domain::power::PowerMetrics, SmError};
use sqlx::PgPool;

pub async fn write_power_metric(pool: &PgPool, model: PowerMetrics) -> Result<(), SmError> {
    sqlx::query!(
        // language=PostgrSQL
        r#"
        INSERT INTO power_metrics (
            home_id,
            ts,
            power,
            min_power,
            average_power,
            max_power,
            last_meter_consumption,
            last_meter_production,
            accumulated_consumption,
            accumulated_production,
            accumulated_cost,
            accumulated_production_last_hour,
            accumulated_consumption_last_hour,
            currency
        )
        VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14);
        "#,
        model.home_id,
        model.ts,
        model.power,
        model.min_power,
        model.average_power,
        model.max_power,
        model.last_meter_consumption,
        model.last_meter_production,
        model.accumulated_consumption,
        model.accumulated_production,
        model.accumulated_cost,
        model.accumulated_production_last_hour,
        model.accumulated_consumption_last_hour,
        model.currency
    )
    .execute(pool)
    .await
    .map_err(|err| SmError::SqlExeption(err.to_string()))?;
    Ok(())
}
