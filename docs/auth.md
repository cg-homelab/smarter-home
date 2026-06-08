# Authentication Flow

This project uses JWT-based authentication with backend token issuance and webapp cookie-based session storage.

Auth.js and GitHub OAuth are not part of the current authentication flow.

## Backend auth endpoints

Defined in src/services/api/src/routes/auth.rs:

- POST /auth/login
- POST /auth/signup

Both return an AuthBody that contains an access token.

## Webapp auth API routes

Webapp route handlers proxy auth calls and manage browser session cookie:

- app/api/auth/login/route.ts
- app/api/auth/register/route.ts
- app/api/auth/logout/route.ts
- app/api/auth/session/route.ts

On login/register success, the webapp stores __session as an httpOnly cookie.

## Session cookie behavior

Cookie name: __session

Configured with:

- httpOnly: true
- sameSite: strict
- secure: true in production
- expiration aligned with JWT exp claim

## Backend token validation

Backend request auth extraction uses Claims in src/lib/lib-utils/src/crypto.rs.
Handlers that require auth rely on Claims extraction from Authorization: Bearer <token>.

## Server-to-backend API calls

src/services/webapp/lib/api.ts reads __session and sends Authorization header for server-side requests.

## Security expectations

- Never persist JWT in localStorage.
- Do not expose JWT to client-side logs.
- Keep auth checks server-side for protected operations.
- If auth response contracts change, update this file and docs/backend-guidelines.md.
