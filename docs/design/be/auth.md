# Authentication Design

Authentication is stateless JWT-based.

The server issues HS512-signed access tokens through `jsonwebtoken` using the configured `JWT_SECRET_KEY`. The minimum key length is 64 bytes.

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

API routes use `be/src/middleware/auth.rs`, which validates any Bearer token and inserts an `AuthContext` request extension. Public handlers can accept optional auth context, while protected routes require that extension before dispatch.

Implemented endpoints:

- `POST /api/auth/signup`
- `POST /api/auth/login`
- `GET /api/auth/me`
- `POST /api/auth/logout`
- `POST /api/auth/check-if-user-exists`
- `POST /api/auth/reset-password-request`
- `POST /api/auth/reset-password`
- `GET /api/auth/verify-user-email`
- `GET /api/users/{user_name}`

The same endpoints are also exposed under `/api/v1`.

Passwords must contain at least 9 and at most 256 characters, with at least one ASCII uppercase letter, lowercase letter, digit, and symbol. Passwords are hashed and verified with Argon2id version 0x13 in blocking tasks, using 64 MiB memory, 3 iterations, 1 lane, and a 32-byte output. Password-bearing DTOs derive `Zeroize` and `ZeroizeOnDrop`.

There is no server-side session store. Logout is client-side token discard. Password reset increments `user_auth_token_version`; consumers can use the rich claim for stricter invalidation policies later.

Security-sensitive follow-up work:

- Enforce `user_auth_token_version` during protected request handling if immediate invalidation after password reset is required.
- Add token cleanup jobs after the in-process scheduler is implemented.
- Public auth routes are rate-limited with `tower-governor` in `be/src/router/app.rs`.
- `tower-governor` uses `SmartIpKeyExtractor`; deployments behind reverse proxies must strip untrusted forwarding headers before setting trusted `x-forwarded-for`, `x-real-ip`, or `forwarded` values.
