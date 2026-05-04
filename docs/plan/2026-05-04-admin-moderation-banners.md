# Admin Moderation And Banners Plan

Date: 2026-05-04

## Goal

Give admins and moderators backend-authoritative tools for user/provider oversight, bans, central blog content, and advertisement banners.

## Database

- [ ] Add bans table with UUIDv7 primary key, target user, actor user, reason, scope, active window, revoked fields, and timestamps.
- [ ] Add central blog post table with author, slug, title, body, hero image, publication state, and timestamps.
- [ ] Add advertisement banner table with placement, image, target URL, schedule window, visibility, priority, and audit timestamps.
- [ ] Add indexes for active ban lookup, published central blog reads, and active banner scheduling.

## Backend

- [ ] Add moderation, ban, central blog, and banner domain objects.
- [ ] Add admin/moderator DTOs with `utoipa::ToSchema`.
- [ ] Add repositories for active ban checks, ban/unban workflows, central blog CRUD, and banner CRUD.
- [ ] Add services enforcing admin/moderator authorization.
- [ ] Add controllers under admin route groups.

## Frontend

- [ ] Add admin dashboard overview.
- [ ] Add user/provider moderation list and ban controls.
- [ ] Add central blog management.
- [ ] Add advertisement banner management with schedule controls.

## Tests

- [ ] Repository tests for active/revoked/expired bans.
- [ ] Service tests for role authorization.
- [ ] API tests for admin-only route protection.
- [ ] Fuzz tests for banner schedule windows.
