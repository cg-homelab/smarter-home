-- +goose Up
-- +goose StatementBegin
CREATE TABLE IF NOT EXISTS power_metrics (
  home_id UUID NOT NULL,
  ts TIMESTAMPTZ NOT NULL,
  price DOUBLE PRECISION NOT NULL,
  power DOUBLE PRECISION NOT NULL,
  solar_power DOUBLE PRECISION NOT NULL,
  last_meter_consumption DOUBLE PRECISION NOT NULL,
  last_meter_production DOUBLE PRECISION NOT NULL,
  last_solar_total DOUBLE PRECISION NOT NULL,
  consumption_since_midnight DOUBLE PRECISION NOT NULL,
  production_since_midnight DOUBLE PRECISION NOT NULL,
  solar_since_midnight DOUBLE PRECISION NOT NULL,
  cost_since_midnight DOUBLE PRECISION NOT NULL,
  currency VARCHAR(10) NOT NULL
);

SELECT create_hypertable(
	'power_metrics',
	by_range('ts', INTERVAL '1 month'),
	if_not_exists=>TRUE
);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
DROP TABLE IF EXISTS power_metrics;
-- +goose StatementEnd
