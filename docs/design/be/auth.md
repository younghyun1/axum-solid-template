# Authentication Design

Authentication is stateless JWT-based.

The server issues HS512-signed access tokens through `jsonwebtoken` using the configured `JWT_SECRET_KEY`. The minimum key length is 64 bytes.

JWT claims are intentionally rich:

- standard claims: `iss`, `sub`, `aud`, `exp`, `nbf`, `iat`, `jti`
- user metadata: user id, name, email, email verification state, country, language, subdivision
- auth metadata: token version
- role metadata: role id, role name, role type, access level
- ISO timestamps for issue and expiry time

Protected routes use `be/src/middleware/auth.rs`, which validates the Bearer token and inserts an `AuthContext` request extension.

Implemented endpoints:

- `POST /api/auth/signup`
- `POST /api/auth/login`
- `GET /api/auth/me`
- `POST /api/auth/logout`
- `GET /api/auth/is-superuser`
- `POST /api/auth/check-if-user-exists`
- `POST /api/auth/reset-password-request`
- `POST /api/auth/reset-password`
- `GET /api/auth/verify-user-email`
- `GET /api/users/{user_name}`

The same endpoints are also exposed under `/api/v1`.

Passwords are hashed and verified with Argon2id in blocking tasks. Password-bearing DTOs derive `Zeroize` and `ZeroizeOnDrop`.

There is no server-side session store. Logout is client-side token discard. Password reset increments `user_auth_token_version`; consumers can use the rich claim for stricter invalidation policies later.
