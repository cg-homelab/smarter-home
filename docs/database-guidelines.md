# Database Guidelines

Database: TimescaleDB (PostgreSQL)
Migration tooling: SQLx and sqlx-cli

## Source of truth

- Migration files in migrations/ are the source of schema truth.
- Runtime and compile-time query contracts are enforced by SQLx.

## Container data directory

The `timescale/timescaledb-ha` image sets `PGDATA=/home/postgres/pgdata/data`,
not the conventional `/var/lib/postgresql/data`. `docker-compose.yml` mounts the
named volume at that path, and the image is pinned by digest so an upstream retag
cannot move it.

Mounting the conventional path instead is silently destructive: Postgres keeps
using its own `PGDATA` inside the container's writable layer, the volume stays
empty, and all data is lost the next time the container is recreated. Verify with:

```bash
docker exec sm-db ls /home/postgres/pgdata/data   # should list PG_VERSION, base, global
```

## Migration rules

- New migrations should include both up and down files.
- Use defensive SQL where possible:
  - IF NOT EXISTS
  - IF EXISTS
- Keep migrations focused and reversible.
- Verify rollback behavior before merge.

## SQLx operations

Use these recipes during development:

- just db start
- just db status
- just db up
- just db down
- just db reset (destructive, asks for confirmation)
- just db mig-create <name>
- just db prepare-offline

If SQL in src/lib/lib-db changes, just db prepare-offline is required.

Run `just --show db::up` to see the underlying sqlx command behind any recipe.

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
