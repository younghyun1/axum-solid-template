# Master Plan

## Backend Rustdoc Coverage

Scope: all Rust functions in `be/` (`*.rs` files), excluding `be/build.rs` per request.

- [x] Read and apply repository instructions from AGENTS.md.
- [x] Inventory function declarations.
- [x] Prioritize files by size and risk.
- [x] Add rustdoc comments to all Rust functions under `be/src`.
- [x] Keep comments concise but explicit for args, success path, failure path, and side effects.
- [x] Report files changed and exceptions.
- [ ] Run formatting and doc checks.

Progress:

- [x] 2026-05-03: Planning documents created.
- [x] 2026-05-03: One-by-one rustdoc insertion completed for every function in `be/src` using signature-derived docs.
- [x] 2026-05-03: `be/build.rs` left untouched per request.
- [ ] Validation pass pending for the original rustdoc rollout.

## Recent Work

- [x] Diagnose Docker TTRPC startup failure after frontend build and backend builder image build.
- [x] Fix email verification form usability while browser proof-of-work is pending.
- [x] Replace hard-coded email verification seed question IDs with real UUIDv7 values.
- [x] Add admin database reset endpoint and admin-panel reset button.
- [x] Tune email verification browser proof-of-work latency and progress reporting.
- [x] Replace email verification PoW WebCrypto dependency with worker-local SHA-256.
- [x] Fix admin database reset FK failure and reshape admin UI into a dashboard.
- [x] Sync frontend page navigation with browser URLs.
- [x] Repair Solid Router integration, review backend SPA fallback routing, and add Not Found page.
- [x] Add post-signup email verification guidance before sign-in.
- [x] Add five-second flashy hold-to-reset database admin control.
- [x] Fix stale auth state after database reset.
- [x] Implement HttpOnly cookie JWT auth, refresh sessions, and environment-aware CORS/cookie policy.
- [x] Add cookie/CORS environment variables to backend dotenv.
- [x] Make signup pending-verification copy production-oriented.

## Service Marketplace Platform

Scope: evolve the existing cookie-auth, role, DTO, utoipa, Diesel/PostgreSQL, and SolidJS foundation into a role-focused service marketplace.

- [x] Persist master and feature planning documents under `docs/plan`.
- [x] Persist backend and frontend marketplace design conventions under `docs/design`.
- [x] Add shared image/media database model, domain, DTO, repository, service, and API surface.
- [x] Add public provider directory and provider profile/blog model, workflows, and API surface.
- [x] Add abstract payment intent, transaction ledger, and processor event model/workflows.
- [x] Add admin moderation, account ban, central blog, and advertisement banner management.
- [x] Extend OpenAPI/Swagger documentation for every new API DTO.
- [x] Build marketplace frontend shell for public, user, provider, and admin workflows.
- [ ] Add backend repository/service/API/fuzz tests for new behavior.
- [ ] Add frontend strict TypeScript/API/navigation/component tests where project tooling supports it.
- [x] Run `cargo fmt`, `cargo clippy`, non-release backend checks/tests, and `npm run build`.

Progress:

- [x] 2026-05-04: Marketplace plan persisted in `docs/plan/2026-05-04-marketplace-platform-roadmap.md`.
- [x] 2026-05-04: Shared image/media subsystem plan persisted.
- [x] 2026-05-04: Provider directory/profile/blog plan persisted.
- [x] 2026-05-04: Abstract payments ledger plan persisted.
- [x] 2026-05-04: Admin moderation/banner plan persisted.
- [x] 2026-05-04: Backend and frontend design docs created for marketplace work.
- [x] 2026-05-04: Marketplace backend schema, APIs, OpenAPI, repositories, and services implemented.
- [x] 2026-05-04: Marketplace frontend shell and typed API client implemented.
- [x] 2026-05-04: Ran `cargo fmt`, `cargo check`, `cargo clippy`, `cargo test`, `npm run build`, and `npm test`.

## Marketplace Frontend Production Layout

Scope: replace the initial thin marketplace shell with a production-grade listing experience for public provider discovery and provider detail browsing.

- [x] Persist production frontend layout plan under `docs/plan`.
- [x] Audit current marketplace pages, CSS, navigation, and typed DTO usage.
- [x] Rework public provider directory into a listing-site layout with search, filter rail, dense result cards, sorting controls, and trustworthy empty/loading/error states.
- [x] Rework provider detail into a credible profile page with media, service summary, trust signals, blog preview, and clear transaction entry points.
- [x] Keep styling centralized and reusable under `fe/src/styles`.
- [x] Verify strict TypeScript build and frontend tests.
- [x] Commit the production layout slice separately from unrelated generated files.

Progress:

- [x] 2026-05-04: Production frontend layout plan persisted in `docs/plan/2026-05-04-marketplace-frontend-production-layout.md`.
- [x] 2026-05-04: Public provider directory and profile pages rebuilt with listing-site structure.
- [x] 2026-05-04: Ran `npm run build` and `npm test`.

## Marketplace Production Hardening

Scope: turn the marketplace from a functional first slice into a production-grade service marketplace surface with rich blog authoring, durable full-text search, single-server cache persistence, and stronger API/UI workflows.

- [x] Persist production hardening plan documents under `docs/plan`.
- [x] Confirm current library choices against primary documentation.
- [x] Add production markdown authoring plan and implementation for provider/admin blog workflows.
- [x] Add sanitized markdown rendering model for public blog content.
- [ ] Add Tantivy-backed search indexing for providers and blog posts.
- [ ] Add single-server RAM cache plus disk persistence for hot marketplace reads.
- [ ] Add backend APIs and OpenAPI docs for search, cache-backed public reads, blog updates, and moderation transitions.
- [ ] Upgrade provider/admin frontend dashboards around rich authoring and content management.
- [ ] Add focused backend and frontend tests for the new production behavior.
- [ ] Run backend/frontend verification and commit in logical slices.

Progress:

- [x] 2026-05-04: Production hardening master plan persisted.
- [x] 2026-05-04: Markdown editor, search/cache, API hardening, and dashboard hardening subplans created.
- [x] 2026-05-04: Added Comrak/Ammonia backend rendering and Milkdown Crepe provider/admin blog editors.
- [x] 2026-05-04: Ran `cargo check`, markdown renderer unit tests, `npm run build`, and `npm test` for the markdown slice.
