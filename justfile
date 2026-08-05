# Smarter Home task runner. Requires just >= 1.48 — https://just.systems
#
#   just db …      migrations and the SQLx offline cache
#   just api …     Rust API service
#   just web …     Next.js webapp
#   just docker …  Docker Compose stack
#
# `check` recipes are read-only. `fmt` / `fix` recipes write files.
# Run `just` for the recipe list, `just --show db::up` for a recipe body.

set dotenv-load := true
set windows-shell := ['powershell.exe', '-NoLogo', '-Command']

# Database migrations and the SQLx offline cache
[group('modules')]
mod db 'just/db.just'

# Rust API service
[group('modules')]
mod api 'just/api.just'

# Next.js webapp
[group('modules')]
mod web 'just/web.just'

# Docker Compose stack
[group('modules')]
mod docker 'just/docker.just'

# Show every recipe, including module recipes
[group('meta')]
help:
    @just --list --list-submodules

# Verify required local tooling is installed
[group('meta')]
system-validate:
    @echo "cargo   {{ require('cargo') }}"
    @echo "bun     {{ require('bun') }}"
    @echo "docker  {{ require('docker') }}"
    @echo "just    {{ require('just') }}"
    @echo "checking sqlx-cli (missing? run: just install-sqlx)"
    @echo "sqlx    {{ require('sqlx') }}"
    @just --version

# Install sqlx-cli (one time, ~2-3 minutes)
[group('meta')]
install-sqlx:
    cargo install sqlx-cli --no-default-features --features rustls,postgres

# Validate tooling, then install webapp dependencies
[group('meta')]
deps-install: system-validate web::install

# Read-only validation gate — run before opening a pull request
[group('validate')]
check: api::check web::check

# Auto-format and auto-fix (writes files)
[group('validate')]
fix: api::fmt web::fix
