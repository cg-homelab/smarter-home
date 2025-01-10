-- +goose Up
-- +goose StatementBegin
CREATE TABLE IF NOT EXISTS homes (
  id UUID PRIMARY KEY,
  name TEXT NOT NULL,
  address TEXT
);

CREATE TABLE IF NOT EXISTS apps (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL,
  home_id UUID NOT NULL REFERENCES homes(id),
  key TEXT NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
  last_updated TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
SELECT 'down SQL query';
DROP TABLE IF EXISTS apps;
DROP TABLE IF EXISTS homes;
-- +goose StatementEnd
