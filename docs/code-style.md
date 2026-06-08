# Code Style

## Rust (backend and libs)

- Format code with cargo fmt.
- Validate code quality with cargo clippy --bin api.
- Keep modules focused by domain and scope.
- Prefer explicit error handling over panic-driven flow.
- Keep public types and functions documented with Rust doc comments.
- Add unit tests for utility logic in src/lib/lib-utils.

## TypeScript and Next.js (webapp)

- Validate with bun run lint and bunx tsc --noEmit.
- Keep backend communication out of random client components.
- Route backend communication through server actions and src/services/webapp/lib/api.ts.
- Keep reusable types in src/services/webapp/types/.
- Keep reusable UI primitives in shared components (prefer shadcn-based primitives).

## SQL and migrations

- Keep migrations small, explicit, and reversible.
- Use IF NOT EXISTS / IF EXISTS where possible.
- Avoid ambiguous SQL in performance-sensitive paths.
- Keep naming readable and consistent across schema and query aliases.
- For lib-db query changes, regenerate SQLx metadata with make db-prepare-offline.
- On Windows without make, run cargo sqlx prepare --workspace.
