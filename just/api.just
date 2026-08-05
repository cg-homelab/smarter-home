# Rust API recipes. `check` is read-only; `fmt` writes files.

set working-directory := '..'
set windows-shell := ['powershell.exe', '-NoLogo', '-Command']

# Show api recipes
[group('api')]
help:
    @just --list api

# Run the API locally, outside Docker
[group('api')]
dev:
    cargo run --bin api

# Fast lint of the api binary
[group('api')]
lint:
    cargo clippy --bin api

# Lint the whole workspace including tests — the pre-PR standard
[group('api')]
lint-ci:
    cargo clippy --workspace --all-targets

# Format Rust code (writes files)
[group('api')]
fmt:
    cargo fmt --all

# Verify formatting without writing files
[group('api')]
fmt-check:
    cargo fmt --all -- --check

# Run the Rust test suite: just api test -- --nocapture
[group('api')]
test *args:
    cargo test --workspace {{ args }}

# Read-only backend gate: formatting, workspace lint, tests
[group('api')]
check: fmt-check lint-ci test
