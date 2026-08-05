use lib_models::{
    domain::weather::{
        DomainWeatherForecast, DomainWeatherForecastsIngest, DomainWeatherObservation,
        DomainWeatherObservationsIngest,
    },
    error::Error,
};
use sqlx::types::JsonValue;
use uuid::Uuid;

use crate::Db;

pub struct WeatherObservation;
pub struct WeatherForecast;

impl WeatherObservation {
    pub async fn insert_observations(
        db: &Db,
        payload: &DomainWeatherObservationsIngest,
    ) -> Result<u64, Error> {
        if payload.observations.is_empty() {
            return Ok(0);
        }

        let (location_hash_high, location_hash_medium, location_hash_low) =
            Self::get_home_hashes(db, payload.home_id).await?;

        let mut tx = db.pool.begin().await?;
        let mut rows_affected = 0;

        for observation in &payload.observations {
            rows_affected += Self::insert_single_observation(
                &mut tx,
                payload.home_id,
                &location_hash_high,
                &location_hash_medium,
                &location_hash_low,
                observation,
            )
            .await?;
        }

        tx.commit().await?;
        Ok(rows_affected)
    }

    async fn get_home_hashes(db: &Db, home_id: Uuid) -> Result<(String, String, String), Error> {
        let home_hashes: (String, String, String) = sqlx::query_as(
            r#"
            SELECT location_hash_high, location_hash_medium, location_hash_low
            FROM homes
            WHERE id = $1
            "#,
        )
        .bind(home_id)
        .fetch_one(&db.pool)
        .await?;

        Ok(home_hashes)
    }

    async fn insert_single_observation(
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        home_id: Uuid,
        location_hash_high: &str,
        location_hash_medium: &str,
        location_hash_low: &str,
        observation: &DomainWeatherObservation,
    ) -> Result<u64, Error> {
        let provider = observation
            .provider
            .as_deref()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or("openweather");
        let raw_payload = observation.raw.clone().unwrap_or(JsonValue::Null);

        let result = sqlx::query(
            r#"
            INSERT INTO weather_observations (
                home_id,
                ts,
                provider,
                location_hash_high,
                location_hash_medium,
                location_hash_low,
                temp_c,
                clouds_pct,
                rain_mm_1h,
                wind_ms,
                pressure_hpa,
                humidity_pct,
                uvi,
                solar_irradiance_wm2,
                weather_code,
                raw
            )
            VALUES (
                $1, $2, $3, $4, $5, $6,
                $7, $8, $9, $10, $11, $12, $13, $14, $15, $16
            )
            ON CONFLICT (home_id, provider, ts)
            DO UPDATE SET
                location_hash_high = EXCLUDED.location_hash_high,
                location_hash_medium = EXCLUDED.location_hash_medium,
                location_hash_low = EXCLUDED.location_hash_low,
                temp_c = EXCLUDED.temp_c,
                clouds_pct = EXCLUDED.clouds_pct,
                rain_mm_1h = EXCLUDED.rain_mm_1h,
                wind_ms = EXCLUDED.wind_ms,
                pressure_hpa = EXCLUDED.pressure_hpa,
                humidity_pct = EXCLUDED.humidity_pct,
                uvi = EXCLUDED.uvi,
                solar_irradiance_wm2 = EXCLUDED.solar_irradiance_wm2,
                weather_code = EXCLUDED.weather_code,
                raw = EXCLUDED.raw,
                ingested_at = NOW()
            "#,
        )
        .bind(home_id)
        .bind(observation.ts)
        .bind(provider)
        .bind(location_hash_high)
        .bind(location_hash_medium)
        .bind(location_hash_low)
        .bind(observation.temp_c)
        .bind(observation.clouds_pct)
        .bind(observation.rain_mm_1h)
        .bind(observation.wind_ms)
        .bind(observation.pressure_hpa)
        .bind(observation.humidity_pct)
        .bind(observation.uvi)
        .bind(observation.solar_irradiance_wm2)
        .bind(&observation.weather_code)
        .bind(raw_payload)
        .execute(&mut **tx)
        .await?;

        Ok(result.rows_affected())
    }
}

impl WeatherForecast {
    pub async fn insert_forecasts(
        db: &Db,
        payload: &DomainWeatherForecastsIngest,
    ) -> Result<u64, Error> {
        if payload.forecasts.is_empty() {
            return Ok(0);
        }

        let mut tx = db.pool.begin().await?;
        let mut rows_affected = 0;

        for forecast in &payload.forecasts {
            rows_affected +=
                Self::insert_single_forecast(&mut tx, payload.home_id, forecast).await?;
        }

        tx.commit().await?;
        Ok(rows_affected)
    }

    async fn insert_single_forecast(
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        home_id: Uuid,
        forecast: &DomainWeatherForecast,
    ) -> Result<u64, Error> {
        let provider = forecast
            .provider
            .as_deref()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or("openweather");
        let raw_payload = forecast.raw.clone().unwrap_or(JsonValue::Null);

        let result = sqlx::query(
            r#"
            INSERT INTO weather_forecasts (
                home_id,
                provider,
                run_ts,
                valid_ts,
                horizon_hours,
                temp_c,
                clouds_pct,
                rain_mm,
                wind_ms,
                uvi,
                raw
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            ON CONFLICT (home_id, provider, run_ts, valid_ts)
            DO UPDATE SET
                horizon_hours = EXCLUDED.horizon_hours,
                temp_c = EXCLUDED.temp_c,
                clouds_pct = EXCLUDED.clouds_pct,
                rain_mm = EXCLUDED.rain_mm,
                wind_ms = EXCLUDED.wind_ms,
                uvi = EXCLUDED.uvi,
                raw = EXCLUDED.raw,
                ingested_at = NOW()
            "#,
        )
        .bind(home_id)
        .bind(provider)
        .bind(forecast.run_ts)
        .bind(forecast.valid_ts)
        .bind(forecast.horizon_hours)
        .bind(forecast.temp_c)
        .bind(forecast.clouds_pct)
        .bind(forecast.rain_mm)
        .bind(forecast.wind_ms)
        .bind(forecast.uvi)
        .bind(raw_payload)
        .execute(&mut **tx)
        .await?;

        Ok(result.rows_affected())
    }
}
