# Marketplace Production Hardening Plan

Date: 2026-05-04

## Goal

Make the marketplace production-grade across backend APIs and frontend workflows. The first implementation slice proved the data model and route shape; this track adds the authoring, search, cache, moderation, and UI polish required for a real service listing product.

## Workstreams

- Markdown authoring and rendering for provider/admin blog content.
- Tantivy-backed search for providers, provider posts, central blog posts, and eventually banners.
- Single-server cache with RAM acceleration and disk persistence for hot public reads.
- API hardening for search, moderation transitions, blog update/delete, image lifecycle, and payment oversight.
- Frontend dashboard hardening for provider content operations and admin review operations.

## Execution Order

- [x] Persist this plan and subplans before implementation.
- [ ] Confirm selected libraries using primary docs.
- [ ] Implement markdown model/rendering/editor slice.
- [ ] Implement Tantivy index service and search API slice.
- [ ] Implement cache layer and use it in public marketplace reads.
- [ ] Fill missing provider/admin API operations.
- [ ] Upgrade provider/admin frontend dashboards.
- [ ] Add tests around rendering, indexing, cache coherence, and route permissions.
- [ ] Run `cargo fmt`, `cargo clippy`, `cargo test`, `npm run build`, and `npm test`.

## Slices And Commits

Use small commits at stable points:

- `docs: plan marketplace hardening`
- `feature: add markdown blog editor`
- `feature: add marketplace search`
- `feature: add marketplace cache`
- `feature: harden marketplace dashboards`
- `test: cover marketplace hardening`

Commit messages should follow the repo's existing concise style.
