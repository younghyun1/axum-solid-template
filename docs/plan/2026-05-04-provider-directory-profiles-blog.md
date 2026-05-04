# Provider Directory, Profiles, And Blog Plan

Date: 2026-05-04

## Goal

Expose a public provider directory while giving service providers authenticated tools to maintain profile, media, and personal blog content.

## Database

- [ ] Add provider profile table with UUIDv7 primary key, owning user FK, display fields, slug, biography, service areas, contact policy, publication status, moderation status, and timestamps.
- [ ] Add user profile extension table for user-controlled public/private profile details.
- [ ] Add provider blog post table with provider FK, slug, title, excerpt, body, visibility, moderation status, hero image FK, published timestamp, and timestamps.
- [ ] Add indexes for public directory filtering, slug lookup, and provider blog listing.

## Backend

- [ ] Add domain models for provider profiles, user profile extensions, blog posts, publication state, and moderation state.
- [ ] Add DTOs for public directory cards, profile detail, provider profile editing, blog editing, and blog reads.
- [ ] Add repositories that fetch directory cards without N+1 image/profile lookups.
- [ ] Add services for public reads, provider ownership updates, slug generation, and blog publication rules.
- [ ] Add controllers for public provider reads and authenticated provider management.

## Frontend

- [ ] Add public provider listing page with filters and provider cards.
- [ ] Add provider detail page with profile, image gallery, and blog list.
- [ ] Add provider dashboard pages for profile editing, media state, and personal blog posts.

## Tests

- [ ] Repository tests for slug uniqueness and published-only public queries.
- [ ] Service tests for provider ownership and moderation behavior.
- [ ] API permission tests for public/user/provider/admin access paths.
- [ ] Fuzz tests for slug generation.
