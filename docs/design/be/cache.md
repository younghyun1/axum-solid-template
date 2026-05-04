# Backend Marketplace Cache Design

The marketplace cache is designed for single-server on-prem deployment. It uses an in-memory hot layer and a disk-backed persistent layer, not Redis or Valkey.

Cache keys are deterministic strings built from a version prefix, resource kind, path parameters, and normalized query values. Cached values are typed DTO JSON with a schema version, creation timestamp, and TTL.

Read flow:

1. Try RAM.
2. Try disk and repopulate RAM on hit.
3. Query PostgreSQL, render DTO, write RAM and disk.

Mutation services invalidate related keys after the database transaction commits. Invalidations are explicit by resource kind and slug/id. Broad namespace invalidation is acceptable for low-volume admin actions but hot public paths should use targeted keys.

All cache events use structured tracing fields for key, source, age, ttl, and result.
