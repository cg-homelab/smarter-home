---
name: pre-pr
description: Run the full CI-equivalent validation gate before opening a pull request — Rust fmt/clippy/tests, webapp lint and typecheck, SQLx offline cache freshness, and migration rollback safety.
disable-model-invocation: true
---

# Pre-PR validation gate

Mirrors `.github/workflows/validate-backend.yml` and `validate-frontend.yml` so failures surface locally instead of in CI.

## Scope first

Determine what actually changed, and only run the relevant sections:

```bash
git fetch -q origin main
git diff --name-only origin/main...HEAD
```

- Rust files (`**/*.rs`, `Cargo.toml`, `Cargo.lock`) → run Backend.
- Anything under `src/services/webapp/` → run Frontend.
- Anything under `src/lib/lib-db/` → run SQLx offline cache.
- Anything under `migrations/` → run Migrations.

Run the sections that apply, in the order below. Report every failure with the exact command and its output — do not summarize away compiler or lint errors.

## Backend

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets
cargo test --workspace
```

- `cargo fmt` failures: run `cargo fmt --all` and re-check.
- Clippy warnings in files touched by this branch must be fixed. Pre-existing warnings in untouched files may be left alone — say so explicitly rather than fixing them silently.

## Frontend

```bash
cd src/services/webapp && bun install --frozen-lockfile
cd src/services/webapp && bun run lint
cd src/services/webapp && bunx tsc --noEmit
```

## SQLx offline cache

Required by CLAUDE.md whenever queries in `src/lib/lib-db` change — CI builds with `SQLX_OFFLINE`, so a stale cache fails the build.

```bash
make db-start
make db-prepare-offline
git status --short .sqlx/
```

If `.sqlx/` is dirty afterwards, the cache was stale: stage those files as part of the PR.

## Migrations

Verify rollback safety for every new migration:

```bash
make db-up
make db-down
make db-up
```

Each new migration needs both an up and a down file, and should use idempotent patterns (`IF NOT EXISTS`, `IF EXISTS`).

## Docs check

If the diff touches `src/services/api/src/routes/`, confirm:

- `#[utoipa::path(...)]` annotations match the handler's actual inputs, outputs, and auth.
- New paths are registered in the OpenApi derive in `src/services/api/src/main.rs`.
- `docs/auth.md` and `docs/backend-guidelines.md` reflect any auth or contract change.

If the diff changes commands, folder paths, or workflows, update `README.md` and the matching file under `docs/` in the same PR.

## Report

Finish with a short pass/fail table of the sections that ran, then list what is left to fix. Do not open the PR — that is a separate, explicit step.
