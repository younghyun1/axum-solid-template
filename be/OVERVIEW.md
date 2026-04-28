# Backend Overview For Future Agents

Read this file first when resuming backend work. It is intentionally terse and optimized for code navigation.

## Commands

- Workdir: `/home/cyh/Personal/2026/rust-solid-template/be`
- Format: `cargo fmt`
- Unit tests: `cargo test`
- Lints: `cargo clippy`
- Do not run release builds for routine checks.

## Hard Rules

- No `unwrap()`, `expect()`, `panic!`, `todo!`, or `unimplemented!` in handwritten code.
- Handle `Option` and `Result` explicitly with `match`; use `if let` only when it is clearly simpler or clippy pushes that way.
- Structured tracing only: `error!(user_id = %user_id, error = %error, "Message")`.
- API request/response structs live in `src/dto`.
- Domain/database structs and domain values live in `src/domain`.
- Repository modules own Diesel queries. Services own workflow logic and transaction boundaries.
- Folders in the Rust module tree must have `mod.rs` with module declarations only.

## Main Flow

- `src/main.rs`: process entrypoint, allocator, server init, server task.
- `src/init/server_init.rs`: dotenv, config, logging, migrations, pool, reference cache, mail sender, HTTP/HTTPS listener.
- `src/router/app.rs`: route graph, CORS, compression, request logging, auth route rate limiting, Swagger UI, static fallback.
- `src/controller`: Axum extractors and utoipa annotations only.
- `src/service`: validation, auth workflows, transactions, JWT/email orchestration.
- `src/repository`: concrete PostgreSQL Diesel functions.
- `src/domain`: persisted structs, role mappings, JWT claims, nutype value wrappers.
- `src/dto`: external API shapes and shared response envelope.
- `src/error/code_error.rs`: canonical API error registry. Add stable codes here, not in a second registry.
- `../docs/design/be/errors.md`: error-code range and stability convention.

## Auth

- Public auth routes are rate-limited in `src/router/app.rs` with `tower-governor`.
- `SmartIpKeyExtractor` reads forwarded IP headers before falling back to peer IP. Only trust this behind a proxy that scrubs client-supplied forwarding headers.
- Signup writes user, user role, and email verification token in one DB transaction.
- Password reset completion updates password and marks the reset token used in one DB transaction.
- Email verification marks the user verified and token used in one DB transaction.
- Password-bearing DTOs derive `Zeroize` and `ZeroizeOnDrop`; password hashing and verification zeroize moved strings inside blocking tasks.

## Database

- PostgreSQL only. MySQL and SQLite config paths fail explicitly.
- Migrations are embedded from `./migrations` and run at startup.
- UUID primary keys use PostgreSQL 18 native `uuidv7()`.
- Column names are table-prefixed, for example `user_id`.
- Reference data cache loads ISO tables once at startup from `src/init/state/cache/reference_data.rs`.

## Current Template Gaps

See `../docs/design/be/backend_template_gaps.md`. Highest-value next items:

- Root workspace manifest.
- Integration test harness with database setup.
- CI for fmt, clippy, tests, migrations, and OpenAPI.
- Job runner implementation for existing `job_runs` schema.
- Config/env reference documentation.
- Broader nutype coverage for persisted IDs after Diesel integration is designed.
