# Database Guidelines

Database: TimescaleDB (PostgreSQL)
Migration tooling: SQLx and sqlx-cli

## Source of truth

- Migration files in migrations/ are the source of schema truth.
- Runtime and compile-time query contracts are enforced by SQLx.

## Migration rules

- New migrations should include both up and down files.
- Use defensive SQL where possible:
  - IF NOT EXISTS
  - IF EXISTS
- Keep migrations focused and reversible.
- Verify rollback behavior before merge.

## SQLx operations

Use these commands during development:

- make db-start
- make db-status
- make db-up
- make db-down
- make db-reset
- make db-prepare-offline

If SQL in src/lib/lib-db changes, db-prepare-offline is required.

On Windows without make, use direct commands:

- docker compose up -d database
- sqlx migrate info
- sqlx migrate run
- sqlx migrate revert
- sqlx database reset
- cargo sqlx prepare --workspace

## Performance best practices

- Design schema for time-series workloads with TimescaleDB hypertables where appropriate.
- Add indexes for common filters, joins, and ordering patterns.
- Avoid full-table scans on hot paths.
- Avoid SELECT * in performance-sensitive queries.
- Use bounded queries (time range or pagination) for dashboard endpoints.
- Review retention and aggregation strategy for high-frequency metric tables.

## Safety and reliability

- Keep migrations deterministic and idempotent where possible.
- Avoid destructive changes without a rollback path.
- Validate migration behavior in local environment before opening PR.
