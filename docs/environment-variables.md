# Environment Variables

Source of truth: .env.example

Copy .env.example to .env before local development.

## Core runtime variables

- LOG_LEVEL:
  - Purpose: backend log verbosity.
  - Typical values: trace, debug, info, warn.
- BACKEND_PORT:
  - Purpose: API port used by src/services/api.
  - Default: 3001.
- APP_ENV:
  - Purpose: backend environment profile label.
  - Default: local.

## Database variables

- DATABASE_URL:
  - Purpose: SQLx/PostgreSQL connection string used by backend database layer.
  - Host note:
    - local host run usually uses localhost.
    - docker-internal run uses the database container hostname.
- DB_HOST:
  - Purpose: host reference used by compose-derived configuration.
  - Default in template: database.
- DB_PORT:
  - Purpose: PostgreSQL port.
  - Default: 5432.
- DB_NAME:
  - Purpose: database name.
- DB_USERNAME:
  - Purpose: database user.
- DB_PASSWORD:
  - Purpose: database password.
- DB_SCHEMA:
  - Purpose: default schema name.
  - Default: public.

## Webapp variables

- FRONTEND_PORT:
  - Purpose: host port exposed for webapp.
  - Default: 3000.
- API_URL:
  - Purpose: server-side base URL used by webapp API proxy/fetch logic.
  - Used in:
    - src/services/webapp/lib/config.ts
    - src/services/webapp/app/api/auth/login/route.ts
    - src/services/webapp/app/api/auth/register/route.ts

## Auth variable

- AUTH_SECRET:
  - Purpose: backend JWT signing/verification secret.
  - Used in: src/lib/lib-utils/src/crypto.rs
  - Required for non-default secure environments.

## Removed/deprecated variables

These legacy keys are no longer part of the active setup:

- NEXTAUTH_URL
- AUTH_GITHUB_ID
- AUTH_GITHUB_SECRET

Do not add them back unless Auth.js/OAuth is reintroduced intentionally.
