# Shared Image Media Subsystem Plan

Date: 2026-05-04

## Goal

Create one image metadata subsystem for provider profiles, user profiles, provider blog posts, central blog posts, and advertisement banners.

## Database

- [ ] Add PostgreSQL enum `image_type`.
- [ ] Add image status and visibility enums if existing conventions do not already cover them.
- [ ] Add `images` table with UUIDv7 primary key, S3-compatible bucket/key metadata, mime type, byte size, dimensions, checksum, upload status, visibility, timestamps, and typed nullable owner FKs.
- [ ] Add check constraints so each `image_type` can only use its valid owner column set.
- [ ] Add indexes for owner lookup, public visibility lookup, and stale upload cleanup.

## Backend

- [ ] Add domain newtypes and lifecycle enums under `be/src/domain`.
- [ ] Add DTOs for image create/complete/list responses under `be/src/dto`.
- [ ] Add PostgreSQL repository functions for create, attach, complete, list by owner, and visibility changes.
- [ ] Add service authorization for user-owned, provider-owned, and admin-owned image records.
- [ ] Add API routes for authenticated image metadata lifecycle and provider/admin image management.

## Tests

- [ ] Migration checks for owner/type constraints.
- [ ] Repository tests for valid and invalid attachments.
- [ ] Service tests for ownership and role authorization.
- [ ] Fuzz tests for mime, dimensions, byte-size, and object-key metadata validation.
