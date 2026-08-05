# Contributing

## Branching

Create branches from main with one of these prefixes:

- feat/<short-description>
- fix/<short-description>
- chore/<short-description>
- docs/<short-description>
- refactor/<short-description>

## Pull requests

- Target main unless instructed otherwise.
- Keep each PR focused on one main topic.
- Include a clear summary of what changed and why.
- Include test/validation output in PR description.
- Update relevant docs when commands, structure, API contracts, auth behavior, or migrations changed.

## Validation checklist

Choose checks based on your change type.

Frontend (src/services/webapp):

- bun run lint
- bunx tsc --noEmit

Backend (Rust):

- cargo fmt
- cargo clippy --bin api
- cargo test (recommended)

Database:

- just db up
- just db down (verify rollback)
- just db prepare-offline (if SQL changed in src/lib/lib-db)

`just check` runs the full read-only gate (backend fmt/lint/test plus webapp lint/typecheck) in one command.

## Additional policy docs

- docs/github-policies.md
- docs/frontend-guidelines.md
- docs/backend-guidelines.md
- docs/database-guidelines.md
- docs/environment-variables.md
