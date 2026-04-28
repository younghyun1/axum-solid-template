# Database Design

The backend currently supports PostgreSQL only.

Database connections use `diesel_async` with the BB8 pool:

- connection type: `AsyncPgConnection`
- pool: `diesel_async::pooled_connection::bb8::Pool`
- minimum idle connections: available parallelism
- max connections: available parallelism times 10
- connection timeout: 2 seconds

MySQL and SQLite configuration values are parsed but fail during pool/migration initialization with an unsupported database error. Service datasource dispatch also returns `DATABASE_UNSUPPORTED` for MySQL and SQLite, so non-Postgres paths fail explicitly if initialization behavior changes later.

Persistence code lives in repository modules. The current concrete repository implementation is PostgreSQL-only under `be/src/repository/**/postgres`. Services call concrete repository functions after a single datasource check at the service boundary; hot query paths do not use trait-object dispatch.

Repository functions accept concrete `&mut AsyncPgConnection` references. This keeps query execution static and lets a later transaction wrapper pass the same connection through several repository calls without changing query code.

Multi-write service workflows should be atomic. Signup, password reset completion, and email verification use explicit `diesel_async::AsyncConnection::transaction` boundaries in the service layer so related writes commit or roll back together.

Migrations run at startup through `diesel_async::AsyncMigrationHarness` on a direct startup connection. Startup logs must include the database type, each applied migration version, and the final applied migration count. A count of zero is still logged so successful no-op migration checks are visible.

The migrations target PostgreSQL 18. UUID primary keys use PostgreSQL's native `uuidv7()` default. No custom UUIDv7 function, `uuid-ossp`, or `pgcrypto` extension is used for UUID generation.

Large ISO language, currency, country, country flag, and subdivision seed migrations are copied from `be/ref_src/migrations`.

Durable scheduled job run history is stored in `job_runs`; see `docs/design/be/jobs.md`.

Template gaps tracked separately in `docs/design/be/backend_template_gaps.md` should be closed before treating this project as a reusable production backend starter.
