# Smarter Home

Canonical instruction source for AI coding agents in this repository.

Energy monitoring and analytics: Rust/Axum API and Next.js webapp over TimescaleDB.

## Scope

Applies to the whole repository. Nested `CLAUDE.md` files add directory-local rules.

Active application services:

- `src/services/api`
- `src/services/webapp`

## Rule Priority

1. Hard rules in this file.
2. Hard rules in nested `CLAUDE.md` files.
3. Soft rules in this file.
4. Soft rules in nested `CLAUDE.md` files.

If two rules conflict, prefer the rule that is safer for users, data, and production stability.

## Hard Rules

### Security and data safety

- Do not commit secrets, tokens, or private keys.
- Do not log sensitive values such as JWT tokens, passwords, or full credential headers.
- Treat auth data as server-only whenever possible.
- `.env` is denied to Read and Edit in `.claude/settings.json`. Use `.env.example` for variable names.

### Database integrity

- Database changes must use SQLx migrations under `migrations/`.
- New migrations must be rollback-safe and include both up and down files.
- Prefer idempotent SQL patterns such as `IF NOT EXISTS` and `IF EXISTS`.
- Any change to SQL queries in `src/lib/lib-db` requires running:
  - `just db prepare-offline`
  - or `cargo sqlx prepare --workspace`
  - then committing the resulting `.sqlx/` changes, or the CI offline build fails.

### API and auth documentation

- New or changed API endpoints in Rust must keep OpenAPI/Utoipa documentation up to date.
- Handler changes that affect input/output/auth behavior must update `docs/auth.md` and `docs/backend-guidelines.md`.

## Soft Rules

- Keep modules scoped by domain and responsibility.
- Prefer reuse over one-off implementations.
- Keep docs concise, current, and example-driven.

## Where Things Go

| Change | Location |
| --- | --- |
| New API endpoint | `src/services/api/src/routes/`, handlers in `src/services/api/src/` |
| Database access and SQLx queries | `src/lib/lib-db` |
| Domain models and shared errors | `src/lib/lib-models` |
| Utilities (with unit tests) | `src/lib/lib-utils` |
| Frontend pages and UI | `src/services/webapp/app/`, `src/services/webapp/components/` |
| Shared frontend types | `src/services/webapp/types/` |
| Schema changes | new migration in `migrations/` |

## Frontend Rules (src/services/webapp)

See `src/services/webapp/CLAUDE.md` for the full set. Summary:

- Validate with `bun run lint` and `bunx tsc --noEmit`.
- All backend communication goes through server actions and `src/services/webapp/lib/api.ts`.
- Prefer SSR/server-rendered flows and safe hydration patterns.
- Prefer shadcn-based UI primitives.

## Backend Rules (Rust)

- Keep Rust documentation comments current for public modules, functions, and types.
- Keep Utoipa annotations current for API handlers.
- Validate with `cargo fmt` and `cargo clippy --bin api`.

## Database Rules

- Engine: TimescaleDB on PostgreSQL.
- State management: SQLx and sqlx-cli migrations.
- Migration files live in `migrations/`.
- Optimize for performance and maintainability (indexes, query shape, retention strategy).

## Validation

Run `/pre-pr` before opening a pull request. It runs the same gate as CI.
The equivalent single command is `just check`, which is read-only and never writes files.

By change type:

| Change type | Commands |
| --- | --- |
| Frontend | `bun run lint`, `bunx tsc --noEmit` |
| Backend | `cargo fmt`, `cargo clippy --bin api`, `cargo test` |
| Query change in `lib-db` | `just db prepare-offline` (or `cargo sqlx prepare --workspace`) |
| Migration | `just db up`, then `just db down` to verify rollback |

## Automation in this repository

- `justfile` + `just/`: the repository task runner. Run `just` for the recipe list. `check` recipes are read-only; `fmt`/`fix` recipes write files. Requires just >= 1.48.
- `.claude/settings.json`: hook and permission configuration, checked into git.
- `.claude/hooks/format-edited-file.sh`: rustfmt/prettier/eslint on every edit.
- `.claude/hooks/sqlx-prepare-reminder.sh`: enforces the offline-cache rule above.
- `.claude/hooks/api-docs-reminder.sh`: enforces the Utoipa/docs rule above.
- `.claude/skills/pre-pr/SKILL.md`: the `/pre-pr` validation gate.

## Documentation Rules

- `README.md` is the quickstart and high-level overview.
- `docs/` contains deep-dive guides and policies.
- When commands, folder paths, architecture, or workflows change, update the related docs in the same pull request.

## Detailed Guides

- `docs/README.md`
- `docs/frontend-guidelines.md`
- `docs/backend-guidelines.md`
- `docs/database-guidelines.md`
- `docs/auth.md`
- `docs/code-style.md`
- `docs/github-policies.md`
- `docs/nextjs-best-practices.md`
- `docs/rust-best-practices.md`
- `docs/environment-variables.md`
