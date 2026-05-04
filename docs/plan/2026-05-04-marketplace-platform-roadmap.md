# Marketplace Platform Roadmap

Date: 2026-05-04

## Goal

Turn the authenticated Rust/Solid template into a service marketplace with public provider browsing, provider-owned profiles/blog/media, user account and payment intent flows, and admin moderation.

## Execution Order

- [x] Persist master, feature, and design planning documents.
- [ ] Inventory existing backend/frontend module structure and conventions.
- [ ] Add database migrations for media, provider profile, profile extension, blogs, bans, payments, processor events, and advertisement banners.
- [ ] Add backend domain objects in `be/src/domain`.
- [ ] Add request/response DTOs in `be/src/dto`, deriving `utoipa::ToSchema`.
- [ ] Add PostgreSQL repositories under `be/src/repository/**/postgres`.
- [ ] Add service workflows under `be/src/service`.
- [ ] Add Axum controllers under `be/src/controller/v1`.
- [ ] Extend `be/src/docs/api_doc.rs`.
- [ ] Add Solid routes, API client methods, centralized styles, and role-specific marketplace pages.
- [ ] Run formatting, lints, checks, and builds.

## Boundaries

- First marketplace slice is directory, profiles, media metadata, blogs, moderation, banners, and abstract payments.
- Booking/request scheduling is out of scope for this slice.
- Payments remain processor-abstract until a provider is selected.
- Uploaded image binary transfer can be presigned-upload based; this slice stores metadata and lifecycle state.

## Cross-Cutting Rules

- Keep Rust files under 300 LOC by splitting modules early.
- Keep frontend styling centralized under `fe/src/styles`.
- Use UUIDv7 database primary keys.
- Prefix table columns with table names.
- Avoid N+1 queries in directory/profile/blog surfaces.
- No `unwrap()` or `expect()` in handwritten backend code.
