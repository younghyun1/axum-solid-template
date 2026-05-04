# Backend Marketplace Cache Design

The marketplace cache is designed for single-server on-prem deployment. It uses a Moka in-memory hot layer and a disk-backed persistent layer, not Redis or Valkey.

Cache keys are deterministic strings built from resource kind, path parameters, and normalized query values. Cached values are typed DTO JSON wrapped in disk records with a schema version and creation timestamp.

Read flow:

1. Try RAM.
2. Try disk and repopulate RAM on hit.
3. Query PostgreSQL, render DTO, write RAM and disk.

Mutation services clear the public cache after the database write commits and rebuild the derived search index for correctness. Broad invalidation is currently used because marketplace mutations are low-volume and single-server; targeted invalidation can be added when write volume justifies the extra complexity.

Admin API:

- `POST /api/v1/marketplace/admin/cache/clear` clears RAM and disk cache entries.

Runtime configuration:

- `MARKETPLACE_CACHE_DIR`, default `./var/marketplace/cache`.
- `MARKETPLACE_CACHE_CAPACITY`, default `512`.
- `MARKETPLACE_CACHE_TTL_SECONDS`, default `120`.

Cache failures on public reads are treated as misses or warnings so PostgreSQL remains authoritative.
