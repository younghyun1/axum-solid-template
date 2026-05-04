# Backend Search Design

Marketplace search is local-first and Tantivy-backed. The search service owns the Tantivy schema, index directory, writer lifecycle, reader reloads, and query parsing.

The canonical data remains PostgreSQL. Tantivy is a derived index that can be rebuilt from database state. Services update the index after successful writes, but admin reindex remains available for recovery.

Public search must enforce visibility before indexing and before returning results. Published provider content requires published state and approved moderation state. Banned or suspended providers are excluded from public results.

Search DTOs live in `be/src/dto`. Domain search documents live in `be/src/domain`. Index code lives under `be/src/service` or a dedicated repository-like infrastructure module, but controllers never call Tantivy directly.

Index directories are configured under local server storage. Startup logs include index path, document count when available, and whether rebuild or reuse occurred.
