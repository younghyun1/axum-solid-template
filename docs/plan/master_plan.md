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
