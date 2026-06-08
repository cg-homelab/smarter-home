# GitHub Policies

## Branch naming

Use descriptive branch names with a prefix:

- feat/<short-description>
- fix/<short-description>
- chore/<short-description>
- docs/<short-description>
- refactor/<short-description>

Examples:

- feat/home-favorite-filter
- fix/db-migration-rollback
- docs/update-agent-guidelines

## Pull request rules

- Open PRs into main unless otherwise coordinated.
- Keep PRs focused on a single concern when possible.
- Include testing evidence for changed areas.
- Include documentation updates when behavior, commands, or structure changed.

## Required checks and validation

Run relevant checks before requesting review:

- Rust:
  - cargo fmt
  - cargo clippy --bin api
  - cargo test (recommended for logic changes)
- Webapp:
  - bun run lint
  - bunx tsc --noEmit
- Database:
  - make db-up
  - make db-down (for migration rollback validation)
  - make db-prepare-offline (for lib-db SQL changes)

## Release and CI notes

- PR merge to main is used by existing release/tag workflows.
- Keep release-related PR labels accurate if semantic versioning automation depends on labels.
