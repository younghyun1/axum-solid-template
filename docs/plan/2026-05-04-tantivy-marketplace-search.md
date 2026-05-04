# Tantivy Marketplace Search Plan

Date: 2026-05-04

## Goal

Use Tantivy for fast local full-text search across public providers and published blog content, with a clear rebuild path and durable on-disk index.

## Backend

- [x] Add marketplace search state that owns Tantivy schema, index directory, readers, writers, and query parsing.
- [x] Index provider display name, headline, bio, slug, and public visibility state.
- [x] Index provider blog and central blog title, excerpt, body, slug, owner/path, and update timestamp.
- [x] Rebuild index at startup and expose admin-only reindex endpoint.
- [ ] Update index after provider/profile/blog mutation workflows without admin reindex.
- [x] Return typed search result DTOs with kind, title, subtitle, slug, snippet, score, and updated timestamp.

## API

- [x] `GET /api/v1/marketplace/search?q=&kind=&limit=`
- [x] `POST /api/v1/marketplace/admin/search/reindex`

## Acceptance Criteria

- [x] Search works without PostgreSQL full-text extensions.
- [x] Search index persists under configured local storage.
- [x] Public search only returns published and approved content.
- [ ] Tests cover indexing, search visibility, and deletion/update refresh.
