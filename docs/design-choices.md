# Design Choices

This document records practical design choices used in this repository.

## Service split

- Rust API in src/services/api for backend HTTP and orchestration.
- Next.js webapp in src/services/webapp for browser-facing UI.
- Shared Rust libraries in src/lib for reuse and clearer boundaries.

## Data layer

- TimescaleDB on PostgreSQL for time-series workloads.
- SQLx for compile-time checked queries and migration handling.
- Migration files under migrations/ as schema source of truth.

## Frontend data flow

- Server functions and server-side request paths are preferred for backend access.
- Shared API transport logic is centralized in src/services/webapp/lib/api.ts.

## API docs

- Utoipa/OpenAPI is used to keep endpoint documentation near handler code.

## Why these choices

- Clear service boundaries improve maintainability.
- SQLx + migrations improves schema/query safety.
- Server-centric auth and data access reduces token exposure risk.
- Centralized docs improve consistency for human contributors and AI agents.
