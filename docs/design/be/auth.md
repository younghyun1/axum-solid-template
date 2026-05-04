# Authentication Design

Authentication uses HttpOnly cookie-backed access JWTs plus durable refresh sessions.

The server issues HS512-signed access tokens through `jsonwebtoken` using the configured
`JWT_SECRET_KEY`. The minimum key length is 64 bytes. Browser clients receive the access JWT in an
`access_token` cookie; frontend JavaScript does not read or store JWT strings.

Login also creates an opaque `refresh_session` cookie. The raw refresh token is never stored in
PostgreSQL; only its SHA-256 hex digest is persisted in `auth_refresh_sessions`. Refresh sessions are
rotated on every `/auth/refresh` call and revoked on logout.

Protected and admin routes reject signed tokens whose user row no longer exists, whose
`users.user_auth_token_version` no longer matches the claim, or whose current role no longer
matches the claim. Database reset and password reset therefore invalidate older access tokens before
their `exp` time.

JWT claims are intentionally rich:

- standard claims: `iss`, `sub`, `aud`, `exp`, `nbf`, `iat`, `jti`
- user metadata: user id, name, email, email verification state, country, language, subdivision
- auth metadata: token version
- role metadata: role id, role name, role type, access level
- ISO timestamps for issue and expiry time

Role claims are part of the signed JWT payload and are validated against the canonical role mapping when a token is decoded. A client can read those claims, but cannot change them without invalidating the HS512 signature.

Canonical roles:

- `admin`: owner and platform administration access.
- `moderator`: elevated moderation access.
- `service_provider`: provider client access for serving user clients.
- `user`: client access for requesting services.
- `guest`: unauthenticated or minimal access.

Role access levels are a coarse authorization hierarchy embedded into the signed JWT:

- `admin`: 4
- `moderator`: 3
- `service_provider`: 2
- `user`: 1
- `guest`: 0

Use exact-role helpers for role-specific workflows, for example `auth_context.is_service_provider()`. Use minimum-role helpers for hierarchical gates, for example `auth_context.has_min_role(RoleType::Moderator)`.

API routes use `be/src/middleware/auth.rs`, which validates the `access_token` cookie and inserts an
`AuthContext` request extension. Public handlers can accept optional auth context, while protected
routes require that extension before dispatch. Optional auth ignores invalid access cookies so
`/auth/logout` and `/auth/refresh` can clear or repair expired browser state.

Credentialed CORS and unsafe-method origin validation are configured from deployment environment.
Local/development deployments allow configured localhost/Vite origins. Production deployments use
explicit allowed origins and secure cookies with `SameSite=Lax`.

Implemented endpoints:

- `POST /api/v1/auth/signup`
- `POST /api/v1/auth/login`
- `POST /api/v1/auth/refresh`
- `GET /api/v1/auth/me`
- `POST /api/v1/auth/logout`
- `POST /api/v1/auth/check-if-user-exists`
- `POST /api/v1/auth/reset-password-request`
- `POST /api/v1/auth/reset-password`
- `GET /api/v1/auth/verify-user-email`

Email verification now uses a challenge step before the token is consumed:

- `GET /api/v1/auth/email-verification/challenge`
- `POST /api/v1/auth/verify-user-email`

The challenge step combines an emailed token, server-issued proof-of-work, a honeypot field,
minimum elapsed time, and a case-insensitive local question answer. Questionnaire data is persisted
in PostgreSQL and mirrored in RAM on `ServerState`; admin mutations write DB first, bump the
questionnaire revision, and refresh RAM before returning.
- `GET /api/v1/users/{user_name}`

Signup accepts `user_role` as either `user` or `service_provider`. Omitted role values default to
`user` so older clients continue creating standard user accounts; public signup never accepts
elevated roles such as `moderator` or `admin`.

Passwords must contain at least 9 and at most 256 characters, with at least one ASCII uppercase letter, lowercase letter, digit, and symbol. Passwords are hashed and verified with Argon2id version 0x13 in blocking tasks, using 64 MiB memory, 3 iterations, 1 lane, and a 32-byte output. Password-bearing DTOs derive `Zeroize` and `ZeroizeOnDrop`.

Refresh sessions are server-side session state. Logout revokes the current refresh session when one
is present and always clears auth cookies. Password reset increments `user_auth_token_version`; both
access-token middleware and refresh-session validation enforce that version against the current
database value.

Security-sensitive follow-up work:

- Add token cleanup jobs after the in-process scheduler is implemented.
- Public auth routes are rate-limited with `tower-governor` in `be/src/router/app.rs`.
- `tower-governor` uses `SmartIpKeyExtractor`; deployments behind reverse proxies must strip untrusted forwarding headers before setting trusted `x-forwarded-for`, `x-real-ip`, or `forwarded` values.
