# Marketplace API And Dashboard Hardening Plan

Date: 2026-05-04

## Goal

Fill the missing production workflow gaps in the provider and admin surfaces.

## Backend API

- Provider blog list/detail/update/archive/publish endpoints.
- Provider image list, attach primary image, hide/delete metadata, and reorder hooks.
- Admin provider/user/blog/banner moderation transition endpoints.
- Admin payment intent and transaction oversight endpoints.
- Public search endpoint and active banner placements.
- OpenAPI coverage for every new request/response DTO.

## Frontend

- Provider dashboard tabs for profile, media, blog editor, posts, and payments.
- Admin dashboard tabs for overview, users/bans, provider moderation, blog moderation, banners, and payments.
- Search-enhanced public directory with result type filters once backend search is available.
- Shared table, status badge, editor, and state components.

## Acceptance Criteria

- Role-specific dashboards can complete real content-management workflows.
- API errors render useful, non-technical feedback.
- Tables and forms stay responsive and information-dense.
- Frontend build and tests remain strict.
