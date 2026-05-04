# Backend Search Design

Marketplace search is local-first and Tantivy-backed. The search state owns the Tantivy schema, index directory, writer lifecycle, reader reloads, and query parsing.

The canonical data remains PostgreSQL. Tantivy is a derived index that can be rebuilt from database state. Services update the index after successful writes, but admin reindex remains available for recovery.

Public search must enforce visibility before indexing and before returning results. Published provider content requires published state and approved moderation state. Banned or suspended providers are excluded from public results.

Search DTOs live in `be/src/dto`. Domain search documents live in `be/src/domain`. Index code lives under `be/src/init/state/search/marketplace` and service entry points live under `be/src/service/marketplace/search.rs`; controllers never call Tantivy directly.

Index directories are configured under local server storage with `MARKETPLACE_SEARCH_INDEX_PATH`, defaulting to `./var/marketplace/search-index`. Startup logs include index path and document count.

Public API:

- `GET /api/v1/marketplace/search?q=&kind=&limit=` returns indexed provider, provider-blog, and central-blog results.
- `POST /api/v1/marketplace/admin/search/reindex` rebuilds the derived index and is moderator/admin-only.

The current implementation rebuilds on startup and through the admin endpoint. Write-through mutation hooks remain the next hardening step so provider/blog/admin writes refresh the index without waiting for reindex.
