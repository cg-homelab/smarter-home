CREATE TABLE IF NOT EXISTS devices (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    home_id UUID NOT NULL REFERENCES homes(id) ON DELETE CASCADE,
    name VARCHAR(120) NOT NULL,
    device_type VARCHAR(100) NOT NULL,
    has_power_meter BOOLEAN NOT NULL DEFAULT FALSE,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (home_id, name),
    UNIQUE (id, home_id)
);

CREATE TABLE IF NOT EXISTS device_metrics (
    device_id UUID NOT NULL,
    home_id UUID NOT NULL,
    ts TIMESTAMPTZ NOT NULL,
    wattage_w DOUBLE PRECISION,
    state TEXT,
    metrics JSONB NOT NULL DEFAULT '{}'::jsonb,
    source TEXT,
    ingested_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT fk_device_metrics_device_home
        FOREIGN KEY (device_id, home_id) REFERENCES devices(id, home_id) ON DELETE CASCADE,
    UNIQUE (device_id, ts)
);

SELECT create_hypertable(
    'device_metrics',
    by_range('ts', INTERVAL '1 month'),
    if_not_exists => TRUE
);

CREATE INDEX IF NOT EXISTS idx_devices_home_id
    ON devices(home_id);

CREATE INDEX IF NOT EXISTS idx_device_metrics_home_ts
    ON device_metrics(home_id, ts DESC);

CREATE INDEX IF NOT EXISTS idx_device_metrics_device_ts
    ON device_metrics(device_id, ts DESC);

CREATE TABLE IF NOT EXISTS weather_observations (
    home_id UUID NOT NULL REFERENCES homes(id) ON DELETE CASCADE,
    ts TIMESTAMPTZ NOT NULL,
    provider VARCHAR(50) NOT NULL DEFAULT 'openweather',
    location_hash_high TEXT NOT NULL,
    location_hash_medium TEXT NOT NULL,
    location_hash_low TEXT NOT NULL,
    temp_c DOUBLE PRECISION,
    clouds_pct DOUBLE PRECISION,
    rain_mm_1h DOUBLE PRECISION,
    wind_ms DOUBLE PRECISION,
    pressure_hpa DOUBLE PRECISION,
    humidity_pct DOUBLE PRECISION,
    uvi DOUBLE PRECISION,
    solar_irradiance_wm2 DOUBLE PRECISION,
    weather_code TEXT,
    raw JSONB NOT NULL DEFAULT '{}'::jsonb,
    ingested_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (home_id, provider, ts)
);

SELECT create_hypertable(
    'weather_observations',
    by_range('ts', INTERVAL '1 month'),
    if_not_exists => TRUE
);

CREATE INDEX IF NOT EXISTS idx_weather_observations_home_ts
    ON weather_observations(home_id, ts DESC);

CREATE INDEX IF NOT EXISTS idx_weather_observations_hash_high_ts
    ON weather_observations(location_hash_high, ts DESC);

CREATE TABLE IF NOT EXISTS weather_forecasts (
    home_id UUID NOT NULL REFERENCES homes(id) ON DELETE CASCADE,
    provider VARCHAR(50) NOT NULL DEFAULT 'openweather',
    run_ts TIMESTAMPTZ NOT NULL,
    valid_ts TIMESTAMPTZ NOT NULL,
    horizon_hours INTEGER NOT NULL,
    temp_c DOUBLE PRECISION,
    clouds_pct DOUBLE PRECISION,
    rain_mm DOUBLE PRECISION,
    wind_ms DOUBLE PRECISION,
    uvi DOUBLE PRECISION,
    raw JSONB NOT NULL DEFAULT '{}'::jsonb,
    ingested_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (home_id, provider, run_ts, valid_ts)
);

SELECT create_hypertable(
    'weather_forecasts',
    by_range('valid_ts', INTERVAL '1 month'),
    if_not_exists => TRUE
);

CREATE INDEX IF NOT EXISTS idx_weather_forecasts_home_valid_ts
    ON weather_forecasts(home_id, valid_ts DESC);

CREATE INDEX IF NOT EXISTS idx_weather_forecasts_home_run_ts
    ON weather_forecasts(home_id, run_ts DESC);
