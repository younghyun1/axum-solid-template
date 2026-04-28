# Backend Layering

HTTP handlers, business services, and persistence repositories are separate layers.

Controller modules live under `be/src/controller`.

- Own Axum extractors, OpenAPI annotations, path/query/body extraction, and response envelope timing.
- Do not contain Diesel queries or business workflows.
- Call service functions directly and wrap `ApiResult<T>` into `ApiResponseResult<T>`.

Service modules live under `be/src/service`.

- Own validation, authorization decisions, use-case orchestration, password hashing, JWT issuance, email queueing, and transaction boundaries when needed.
- Return DTO response values or `ApiError`.
- Dispatch on configured datasource once at the service boundary. Current supported datasource is PostgreSQL. MySQL and SQLite paths return `DATABASE_UNSUPPORTED`.

Repository modules live under `be/src/repository`.

- Own concrete persistence implementation details.
- Current implementation is `repository::auth::postgres`.
- Use concrete functions and concrete `AsyncPgConnection` references. Do not use per-query trait objects, boxed futures, or dynamic datasource dispatch in hot paths.

Domain modules live under `be/src/domain`.

- Own persisted structs, enums, role mappings, and domain values.
- Do not own query execution functions.

DTO modules live under `be/src/dto`.

- Own HTTP request and response shapes only.
