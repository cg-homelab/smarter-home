use chrono::{DateTime, Utc};
use lib_models::{
    domain::device::{
        DomainDevice, DomainDeviceMetric, DomainDeviceMetricsIngest, DomainDevicesIngest,
    },
    error::Error,
};
use serde::{Deserialize, Serialize};
use sqlx::types::JsonValue;
use uuid::Uuid;

use crate::Db;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Device {
    pub id: Uuid,
    pub home_id: Uuid,
    pub name: String,
    pub device_type: String,
    pub has_power_meter: bool,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceMetric {
    pub device_id: Uuid,
    pub home_id: Uuid,
    pub ts: DateTime<Utc>,
    pub wattage_w: Option<f64>,
    pub state: Option<String>,
    pub metrics: JsonValue,
    pub source: Option<String>,
}

impl Device {
    pub async fn upsert_devices(
        db: &Db,
        payload: &DomainDevicesIngest,
    ) -> Result<Vec<DomainDevice>, Error> {
        let mut devices = Vec::with_capacity(payload.devices.len());

        for device in &payload.devices {
            let is_active = device.is_active.unwrap_or(true);

            let row = if let Some(device_id) = device.id {
                sqlx::query_as::<_, Device>(
                    r#"
                    INSERT INTO devices (
                        id,
                        home_id,
                        name,
                        device_type,
                        has_power_meter,
                        is_active
                    )
                    VALUES ($1, $2, $3, $4, $5, $6)
                    ON CONFLICT (id)
                    DO UPDATE SET
                        name = EXCLUDED.name,
                        device_type = EXCLUDED.device_type,
                        has_power_meter = EXCLUDED.has_power_meter,
                        is_active = EXCLUDED.is_active,
                        updated_at = NOW()
                    WHERE devices.home_id = EXCLUDED.home_id
                    RETURNING id, home_id, name, device_type, has_power_meter, is_active
                    "#,
                )
                .bind(device_id)
                .bind(payload.home_id)
                .bind(&device.name)
                .bind(&device.device_type)
                .bind(device.has_power_meter)
                .bind(is_active)
                .fetch_optional(&db.pool)
                .await?
                .ok_or(Error::Unauthorized)?
            } else {
                sqlx::query_as::<_, Device>(
                    r#"
                    INSERT INTO devices (
                        home_id,
                        name,
                        device_type,
                        has_power_meter,
                        is_active
                    )
                    VALUES ($1, $2, $3, $4, $5)
                    ON CONFLICT (home_id, name)
                    DO UPDATE SET
                        device_type = EXCLUDED.device_type,
                        has_power_meter = EXCLUDED.has_power_meter,
                        is_active = EXCLUDED.is_active,
                        updated_at = NOW()
                    RETURNING id, home_id, name, device_type, has_power_meter, is_active
                    "#,
                )
                .bind(payload.home_id)
                .bind(&device.name)
                .bind(&device.device_type)
                .bind(device.has_power_meter)
                .bind(is_active)
                .fetch_one(&db.pool)
                .await?
            };

            devices.push(Self::to_domain(row));
        }

        Ok(devices)
    }

    fn to_domain(device: Device) -> DomainDevice {
        DomainDevice {
            id: device.id,
            home_id: device.home_id,
            name: device.name,
            device_type: device.device_type,
            has_power_meter: device.has_power_meter,
            is_active: device.is_active,
        }
    }
}

impl DeviceMetric {
    pub async fn insert_metrics(
        db: &Db,
        payload: &DomainDeviceMetricsIngest,
    ) -> Result<u64, Error> {
        if payload.metrics.is_empty() {
            return Ok(0);
        }

        let mut tx = db.pool.begin().await?;

        let mut device_ids = payload
            .metrics
            .iter()
            .map(|metric| metric.device_id)
            .collect::<Vec<_>>();
        device_ids.sort();
        device_ids.dedup();

        let device_count: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM devices WHERE home_id = $1 AND id = ANY($2)")
                .bind(payload.home_id)
                .bind(&device_ids)
                .fetch_one(&mut *tx)
                .await?;

        if device_count != device_ids.len() as i64 {
            return Err(Error::Unauthorized);
        }

        let mut rows_affected: u64 = 0;

        for metric in &payload.metrics {
            rows_affected += Self::insert_single_metric(&mut tx, payload.home_id, metric).await?;
        }

        tx.commit().await?;

        Ok(rows_affected)
    }

    async fn insert_single_metric(
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        home_id: Uuid,
        metric: &DomainDeviceMetric,
    ) -> Result<u64, Error> {
        let metrics_payload = metric.metrics.clone().unwrap_or(JsonValue::Null);

        let result = sqlx::query(
            r#"
            INSERT INTO device_metrics (
                device_id,
                home_id,
                ts,
                wattage_w,
                state,
                metrics,
                source
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (device_id, ts)
            DO UPDATE SET
                wattage_w = EXCLUDED.wattage_w,
                state = EXCLUDED.state,
                metrics = EXCLUDED.metrics,
                source = EXCLUDED.source,
                ingested_at = NOW()
            WHERE device_metrics.home_id = EXCLUDED.home_id
            "#,
        )
        .bind(metric.device_id)
        .bind(home_id)
        .bind(metric.ts)
        .bind(metric.wattage_w)
        .bind(&metric.state)
        .bind(metrics_payload)
        .bind(&metric.source)
        .execute(&mut **tx)
        .await?;

        if result.rows_affected() == 0 {
            return Err(Error::Unauthorized);
        }

        Ok(result.rows_affected())
    }
}
