CREATE TABLE 'live_consumptions' (
  home_id SYMBOL capacity 256 CACHE,
  timestamp TIMESTAMP,
  power DOUBLE,
  min_power DOUBLE,
  max_power DOUBLE,
  average_power DOUBLE,
  last_meter_consumption DOUBLE,
  last_meter_production DOUBLE,
  accumulated_consumption_today DOUBLE,
  accumulated_production_today DOUBLE,
  accumulated_consumption_hour DOUBLE,
  accumulated_production_hour DOUBLE,
  current_price DOUBLE,
  accumulated_cost_today DOUBLE
) timestamp (timestamp) PARTITION BY MONTH WAL;

CREATE TABLE 'electricity_prices' (
  home_id SYMBOL CAPACITY 256 CACHE,
  currency SYMBOL CAPACITY 256 CACHE,
  timestamp TIMESTAMP,
  total_by_provider DOUBLE,
  spot DOUBLE,
  tax DOUBLE,
  calculated DOUBLE,
  grid DOUBLE
) timestamp (timestamp) PARTITION BY YEAR WAL;
