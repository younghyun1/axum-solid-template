# Tantivy Marketplace Search Plan

Date: 2026-05-04

## Goal

Use Tantivy for fast local full-text search across public providers and published blog content, with a clear rebuild path and durable on-disk index.

## Backend

- Add a marketplace search service that owns Tantivy schema, index directory, readers, writers, and query parsing.
- Index provider display name, headline, bio, service area, slug, and moderation/publication state.
- Index provider blog and central blog title, excerpt, markdown/plain text body, slug, owner, and publish timestamps.
- Rebuild index at startup when configured, and expose admin-only reindex endpoint.
- Update index after provider/profile/blog mutation workflows.
- Return typed search result DTOs with kind, title, subtitle, slug, snippet, score, and updated timestamp.

## API

- `GET /api/v1/marketplace/search?q=&kind=&limit=`
- `POST /api/v1/marketplace/admin/search/reindex`

## Acceptance Criteria

- Search works without PostgreSQL full-text extensions.
- Search index persists under configured local storage.
- Public search only returns published and approved content.
- Tests cover indexing, search visibility, and deletion/update refresh.
