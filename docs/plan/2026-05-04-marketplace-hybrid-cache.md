# Marketplace Hybrid Cache Plan

Date: 2026-05-04

## Goal

Add a single-server cache layer for hot marketplace reads using RAM for speed and disk persistence for restart resilience.

## Backend

- [x] Add cache configuration for RAM capacity, TTLs, and disk directory.
- [x] Cache public provider directory variants, provider detail payloads, active banners, and rendered blog content.
- [x] Use stable cache keys derived from DTO query parameters and slugs.
- [x] Persist selected payloads on disk as versioned JSON records so the backend can warm cache on restart.
- [x] Invalidate cache after provider profile, image, blog, banner, moderation, or ban mutations.
- [x] Add moderator/admin cache clear endpoint.
- [ ] Log cache hit/miss/write/invalidate events with full source/age/ttl fields.

## Acceptance Criteria

- [x] Public reads check RAM first, disk second, database third.
- [x] Disk cache uses atomic writes.
- [x] Cache values include schema version and creation timestamp.
- [ ] Disk cache file count is pruned to configured capacity.
- [ ] Tests cover key stability, TTL expiration, disk warmup, and invalidation.
