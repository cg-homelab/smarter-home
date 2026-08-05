# Database recipes. TimescaleDB via Docker Compose, migrations via sqlx-cli.
# `set working-directory := '..'` returns to the repository root — submodule
# recipes otherwise run inside just/, which has no migrations/ directory.

set working-directory := '..'
set windows-shell := ['powershell.exe', '-NoLogo', '-Command']

# Show database recipes
[group('database')]
help:
    @just --list db

# Start only the TimescaleDB container
[group('database')]
start:
    docker compose up -d database

# Show migration status
[group('database')]
status:
    sqlx migrate info

# Apply all pending migrations
[group('database')]
up:
    sqlx migrate run

# Roll back the most recent migration
[group('database')]
down:
    sqlx migrate revert

# DESTRUCTIVE: drop, recreate, and re-migrate the database
[group('database')]
[confirm('Drop and recreate the database, losing all data? [y/N]')]
reset:
    sqlx database reset -y

# Create a reversible migration: just db mig-create add_device_table
[group('database')]
mig-create name:
    sqlx migrate add -r {{ name }}

# Regenerate the SQLx offline query cache in .sqlx/
[group('database')]
prepare-offline:
    cargo sqlx prepare --workspace
