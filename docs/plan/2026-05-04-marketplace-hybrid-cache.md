# Marketplace Hybrid Cache Plan

Date: 2026-05-04

## Goal

Add a single-server cache layer for hot marketplace reads using RAM for speed and disk persistence for restart resilience.

## Backend

- Add cache configuration for RAM capacity, TTLs, disk directory, and enable/disable flags.
- Cache public provider directory variants, provider detail payloads, active banners, and rendered blog content.
- Use stable cache keys derived from DTO query parameters and slugs.
- Persist selected payloads on disk as versioned JSON records so the backend can warm cache on restart.
- Invalidate keys after provider profile, image, blog, banner, moderation, or ban mutations.
- Log cache hit/miss/write/invalidate events with structured tracing fields.

## Acceptance Criteria

- Public reads check RAM first, disk second, database third.
- Disk cache is bounded by configuration and uses atomic writes.
- Cache values include schema version and creation timestamp.
- Tests cover key stability, TTL expiration, disk warmup, and invalidation.
