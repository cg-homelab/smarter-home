# Documentation Index

This folder contains detailed engineering documentation that supports README.md.

## What goes where

- README.md:
  - Basic project information
  - Development dependencies and installation
  - How to run locally
  - High-level project structure and technology summary
- docs/:
  - Detailed implementation guides
  - Design and architecture decisions
  - Code style and best practices
  - GitHub contribution policies
  - Auth implementation details
  - Environment variable reference

## Documents

- frontend-guidelines.md: frontend architecture and validation rules.
- backend-guidelines.md: Rust backend and API conventions.
- database-guidelines.md: migrations, SQLx workflow, and performance guidance.
- environment-variables.md: authoritative .env and .env.example variable reference.
- code-style.md: style and structure rules for Rust, TypeScript, and SQL.
- github-policies.md: branch naming and pull request policy.
- design-choices.md: implementation and architecture rationale.
- auth.md: end-to-end authentication flow and security expectations.
- nextjs-best-practices.md: Next.js operational guidance.
- rust-best-practices.md: Rust documentation, tests, and code quality guidance.

## Related root docs

- CONTRIBUTING.md: contributor workflow and validation checklist.
- AGENTS.md: canonical AI agent instruction source.

## Documentation update policy

Update docs in the same pull request when changing:

- Commands in Makefile/package scripts.
- Folder names or major file locations.
- API request/response contracts.
- Authentication or authorization logic.
- Database schema, migrations, or query behavior.
- Build, deployment, or CI behavior.

## Documentation quality rules

- Keep docs action-oriented and specific to this repository.
- Include concrete paths and commands.
- Avoid stale examples and unused alternatives.
- Prefer one canonical command per operation and mention alternatives only when intentionally supported.
