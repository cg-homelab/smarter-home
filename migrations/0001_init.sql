CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    email VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    last_updated TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS sessions (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL UNIQUE,
    session_id VARCHAR NOT NULL,
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE TABLE IF NOT EXISTS homes (
  id UUID PRIMARY KEY,
  name TEXT NOT NULL,
  real_time_consumption BOOLEAN NOT NULL
);

CREATE TABLE IF NOT EXISTS power_metrics (
  home_id UUID NOT NULL,
  ts TIMESTAMPTZ NOT NULL,
  power DOUBLE PRECISION NOT NULL,
  min_power DOUBLE PRECISION NOT NULL,
  average_power DOUBLE PRECISION NOT NULL,
  max_power DOUBLE PRECISION NOT NULL,
  last_meter_consumption DOUBLE PRECISION NOT NULL,
  last_meter_production DOUBLE PRECISION NOT NULL,
  accumulated_consumption DOUBLE PRECISION NOT NULL,
  accumulated_production DOUBLE PRECISION NOT NULL,
  accumulated_cost DOUBLE PRECISION NOT NULL,
  accumulated_production_last_hour DOUBLE PRECISION NOT NULL,
  accumulated_consumption_last_hour DOUBLE PRECISION NOT NULL,
  currency TEXT NOT NULL
);

SELECT create_hypertable(
	'power_metrics', 
	by_range('ts', INTERVAL '1 month'), 
	create_default_indexes=>FALSE, 
	if_not_exists=>TRUE
);
SELECT add_dimension('power_metrics', by_hash('home_id', 50));
