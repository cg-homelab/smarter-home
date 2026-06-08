# Backend Guidelines

Scope: Rust backend and shared Rust libraries.

## Module and crate responsibilities

- src/lib/lib-db:
  - SQLx data access functions
  - database query implementation details
- src/lib/lib-models:
  - domain models
  - shared API-facing data contracts
  - shared error model
- src/lib/lib-utils:
  - shared utility functions
  - required unit tests for utility behavior
- src/services/api:
  - HTTP routing
  - handlers
  - request/response orchestration

## Documentation requirements

- Keep Rust docs current for public modules, types, and functions.
- Prefer module-level docs for non-trivial modules.
- When behavior changes, update docs in the same pull request.

## API documentation requirements

- API handlers should include Utoipa annotations.
- New endpoints must be registered in OpenAPI aggregation.
- Keep request and response types accurate in docs.

## Code quality gates

Run before merge for backend changes:

- cargo fmt
- cargo clippy --bin api

Recommended when relevant:

- cargo test

## SQLx workflow requirements

If a change modifies SQL in src/lib/lib-db:

- run make db-prepare-offline
- or run cargo sqlx prepare --workspace
- on Windows without make, use cargo sqlx prepare --workspace

This ensures SQLx compile-time metadata stays valid.

## Organization and readability

- Keep code separated by scope and domain.
- Keep route handlers thin and delegate logic to lib-db/lib-models/lib-utils where appropriate.
- Avoid large multi-purpose modules when domain-specific modules are clearer.
