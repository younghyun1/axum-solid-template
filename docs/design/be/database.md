# Database Design

The backend currently supports PostgreSQL only.

Database connections use `diesel_async` with the BB8 pool:

- connection type: `AsyncPgConnection`
- pool: `diesel_async::pooled_connection::bb8::Pool`
- minimum idle connections: available parallelism
- max connections: available parallelism times 10
- connection timeout: 2 seconds

MySQL and SQLite configuration values are parsed but fail during pool/migration initialization with an unsupported database error.

Migrations run at startup through `diesel_async::AsyncMigrationHarness` on a direct startup connection.

The migrations target PostgreSQL 18. UUID primary keys use PostgreSQL's native `uuidv7()` default. No custom UUIDv7 function, `uuid-ossp`, or `pgcrypto` extension is used for UUID generation.

Large ISO language, currency, country, country flag, and subdivision seed migrations are copied from `be/ref_src/migrations`.
