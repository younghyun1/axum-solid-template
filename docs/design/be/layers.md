# Backend Layering

HTTP handlers, business services, and persistence repositories are separate layers.

New backend features should add code in this order:

1. Domain structs and enums under `be/src/domain`.
2. Request and response DTOs under `be/src/dto`.
3. Persistence functions under `be/src/repository`.
4. Business workflow functions under `be/src/service`.
5. Axum handlers and utoipa annotations under `be/src/controller`.
6. Route registration under `be/src/router/app.rs`.

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
- Keep repository functions narrow. They should map one query or command to one Rust function and return typed domain values or Diesel errors.

Domain modules live under `be/src/domain`.

- Own persisted structs, enums, role mappings, and domain values.
- Do not own query execution functions.
- Do not expose HTTP-only shapes from this layer.

DTO modules live under `be/src/dto`.

- Own HTTP request and response shapes only.
- Request DTOs containing credentials or tokens should derive `Zeroize` and `ZeroizeOnDrop`.
- DTOs intended for OpenAPI output should derive `utoipa::ToSchema`.

Cross-cutting template rules:

- All API handlers return `ApiResponse<T>` or `ApiResponseResult<T>`.
- All fallible workflows return `ApiResult<T>` from the service boundary.
- Multi-write workflows should use one explicit transaction boundary in the service layer once the transaction helper is added; repository functions should stay reusable inside and outside transactions.
- Folders that participate in the Rust module tree should have a `mod.rs` containing only module declarations.
