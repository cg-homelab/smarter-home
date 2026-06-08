# Authentication Flow

This project uses short-lived JWT access tokens with rotating refresh tokens.
The backend issues and validates tokens, and the webapp stores them in httpOnly cookies.

Auth.js and GitHub OAuth are not part of the current authentication flow.

## Backend auth endpoints

Defined in src/services/api/src/routes/auth.rs:

- POST /auth/login
- POST /auth/signup
- POST /auth/refresh
- POST /auth/logout

Login, signup, and refresh return AuthBody with:

- accessToken
- refreshToken
- tokenType
- expiresIn

Logout revokes the provided refresh token.

## Webapp auth API routes

Webapp route handlers proxy auth calls and manage browser session cookie:

- app/api/auth/login/route.ts
- app/api/auth/register/route.ts
- app/api/auth/refresh/route.ts
- app/api/auth/logout/route.ts
- app/api/auth/session/route.ts

On login/register/refresh success, the webapp stores:

- __session (access token)
- __refresh (refresh token)

## Session cookie behavior

Cookie names:

- __session
- __refresh

Configured with:

- httpOnly: true
- sameSite: strict
- secure: true in production
- __session expiration aligned with JWT exp claim
- __refresh expiration aligned with AUTH_REFRESH_TTL_SECS

## Backend token validation

Backend request auth extraction uses Claims in src/lib/lib-utils/src/crypto.rs.
Handlers that require auth rely on Claims extraction from Authorization: Bearer <token>.

Refresh tokens are stored hashed in the database and rotated on each successful refresh.

## Server-to-backend API calls

src/services/webapp/lib/api.ts reads __session and sends Authorization header for server-side requests.

If backend returns 401 and __refresh is available, apiFetch attempts one refresh and one retry.

src/services/webapp/middleware.ts proactively refreshes tokens on protected app routes when access token is missing or expired.

## Security expectations

- Never persist JWT in localStorage.
- Do not expose JWT to client-side logs.
- Keep auth checks server-side for protected operations.
- If auth response contracts change, update this file and docs/backend-guidelines.md.
