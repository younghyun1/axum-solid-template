# Backend Template Gaps

This project is a useful backend seed, but it is not yet a complete reusable backend template. The items below are the remaining backend-template work after the current auth, API envelope, database, logging, email, and Swagger foundations.

## Must Have

- Root workspace manifest: add a repository-level `Cargo.toml` workspace so backend binaries, one-time Rust scripts, and future crates share dependency and lint configuration.
- Integration test harness: add `be/tests` with database-backed tests, migration setup, and endpoint tests against the Axum router.
- CI: add formatting, clippy, tests, migration validation, and OpenAPI generation checks.
- Job runner implementation: `job_runs` exists, but `be/src/job` only declares the module. Implement the in-process Tokio scheduler described in `docs/design/be/jobs.md`.
- Configuration documentation: add a single env-var reference covering defaults, required production values, local examples, and unsafe local-only settings.
- Request validation convention: centralize validation helpers and document when validation belongs in DTO parsing, service code, or database constraints.
- Auth hardening: add optional token-version checks on protected routes, token cleanup, and audit logging for security-sensitive auth events.
- Database model newtyping: expand `nutype` coverage to persisted IDs after the Diesel integration shape is designed.

## Should Have

- Health/readiness split: keep `/api/v1/healthcheck` simple, then add readiness checks that verify database and cache initialization state.
- Observability baseline: add request IDs, log correlation, structured startup config logging with redacted secrets, and documented log retention behavior.
- OpenAPI artifact flow: generate and validate OpenAPI JSON in CI so frontend clients can depend on stable API descriptions.
- Seed/admin workflow: add a documented first-admin creation path as a Rust binary in the workspace.
- Static asset build path: document or automate how `/fe` is built before embedding into the backend binary.
- Reference data cache split: `be/src/init/state/cache/reference_data.rs` is above the 300 LOC target and should be split by data family and lookup helpers.
- Permission enforcement: roles and permissions tables exist, but authorization currently uses role hierarchy only. Add policy helpers before adding feature-specific permissions.
- Deployment docs: add local systemd, reverse proxy, TLS certificate, database backup, and migration rollback guidance for the single-server deployment target.

## Nice To Have

- Metrics endpoint: expose process, HTTP, DB pool, and job metrics after the logging baseline is stable.
- Graceful shutdown: handle termination signals, stop accepting new requests, drain in-flight requests, and stop jobs cleanly.
- Email template system: replace generic token messages with branded templates and local preview tests.
- API client generation: generate a TypeScript client from OpenAPI once frontend conventions are stable.
- Load-testing scripts: add repeatable smoke and light load tests for login, signup, healthcheck, and static asset serving.
- Module hygiene cleanup: remove unused empty source directories or add real module declarations once those folders become part of the Rust module tree.
