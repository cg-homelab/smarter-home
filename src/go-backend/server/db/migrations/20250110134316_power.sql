-- +goose Up
-- +goose StatementBegin
CREATE TABLE IF NOT EXISTS live_energy_measurement (
  home_id UUID NOT NULL,
  ts TIMESTAMPTZ NOT NULL,
  meter_power DOUBLE PRECISION NOT NULL,
  meter_imported DOUBLE PRECISION NOT NULL,
  meter_exported DOUBLE PRECISION NOT NULL,
  meter_l1 DOUBLE PRECISION,
  meter_l2 DOUBLE PRECISION,
  meter_l3 DOUBLE PRECISION
);

CREATE TABLE IF NOT EXISTS live_energy_price (
  home_id UUID NOT NULL,
  ts TIMESTAMPTZ NOT NULL,
  total DOUBLE PRECISION NOT NULL,
  energy DOUBLE PRECISION NOT NULL,
  tax DOUBLE PRECISION NOT NULL,
  grid DOUBLE PRECISION,
  support DOUBLE PRECISION,
  level VARCHAR(20) -- VERY_CHEAP(<60%), CHEAP(60%-90%), NORMAL(90%-115%), EXPENSIVE(115%-140%), VERY_EXPENSIVE(<140%)
);


SELECT create_hypertable(
	'live_energy_measurement',
	by_range('ts', INTERVAL '1 month'),
	if_not_exists=>TRUE
);

SELECT create_hypertable(
	'live_energy_price',
	by_range('ts', INTERVAL '1 year'),
	if_not_exists=>TRUE
);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
DROP TABLE IF EXISTS live_energy_measurement;
DROP TABLE IF EXISTS live_energy_price;
-- +goose StatementEnd
