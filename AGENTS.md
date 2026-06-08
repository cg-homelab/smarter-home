# Smarter Home Agent Instructions

This file is the canonical instruction source for AI coding agents in this repository.

## Scope

These instructions apply to the whole repository, with additional local rules in nested instruction files.

Current active application services in this workspace:

- src/services/api
- src/services/webapp

## Rule Priority

1. Hard rules in this file.
2. Hard rules in nested instruction files.
3. Soft rules in this file.
4. Soft rules in nested instruction files.

If two rules conflict, prefer the rule that is safer for users, data, and production stability.

## Hard Rules

### Security and data safety

- Do not commit secrets, tokens, or private keys.
- Do not log sensitive values such as JWT tokens, passwords, or full credential headers.
- Treat auth data as server-only whenever possible.

### Database integrity

- Database changes must use SQLx migrations under migrations/.
- New migrations must be rollback-safe and include both up and down files.
- Prefer idempotent SQL patterns such as IF NOT EXISTS and IF EXISTS.
- Any change to SQL queries in src/lib/lib-db requires running:
  - make db-prepare-offline
  - or cargo sqlx prepare --workspace

### API and auth documentation

- New or changed API endpoints in Rust must keep OpenAPI/Utoipa documentation up to date.
- Handler changes that affect input/output/auth behavior must update docs/auth.md and docs/backend-guidelines.md.

## Soft Rules

- Keep modules scoped by domain and responsibility.
- Prefer reuse over one-off implementations.
- Keep docs concise, current, and example-driven.

## Documentation Rules

- README.md is the quickstart and high-level overview.
- docs/ contains deep-dive guides and policies.
- When commands, folder paths, architecture, or workflows change, update the related docs in the same pull request.
- Keep AGENTS.md and .github/copilot-instructions.md aligned when agent-facing rules, workflows, or project structure guidance changes.

## Frontend Rules (src/services/webapp)

- Validate frontend changes with:
  - bun run lint
  - bunx tsc --noEmit
- All backend communication must go through server actions and src/services/webapp/lib/api.ts.
- Keep reusable types in src/services/webapp/types/.
- Prefer SSR/server-rendered flows and safe hydration patterns.
- Show loading states for async UI with loading UI or skeleton components.
- Reuse src/services/webapp/components/ui/skeleton.tsx for placeholder loading UI before creating custom skeleton variants.
- Prefer shadcn-based UI primitives and add with:
  - bunx --bun shadcn@latest add <component>
- Dialogs such as delete/confirm should be reusable components.

## Backend Rules (Rust)

- Crate responsibilities:
  - src/lib/lib-db: database access and SQLx queries.
  - src/lib/lib-models: domain models and shared errors.
  - src/lib/lib-utils: utilities, with unit tests.
  - src/services/api: handlers, routing, and HTTP concerns.
- Keep Rust documentation comments current for public modules/functions/types.
- Keep Utoipa annotations current for API handlers.
- Validate backend changes with:
  - cargo fmt
  - cargo clippy --bin api

## Database Rules

- Database engine: TimescaleDB on PostgreSQL.
- State management: SQLx and sqlx-cli migrations.
- Keep migration files in migrations/.
- Optimize for performance and maintainability (indexes, query shape, retention strategy).

## Validation Checklist by Change Type

- Frontend change:
  - bun run lint
  - bunx tsc --noEmit
- Backend change:
  - cargo fmt
  - cargo clippy --bin api
- Database query change in lib-db:
  - make db-prepare-offline
- Migration change:
  - make db-up
  - make db-down (verify rollback)

## Detailed Guides

- docs/README.md
- docs/frontend-guidelines.md
- docs/backend-guidelines.md
- docs/database-guidelines.md
- docs/auth.md
- docs/code-style.md
- docs/github-policies.md
- docs/nextjs-best-practices.md
- docs/rust-best-practices.md
- docs/environment-variables.md
