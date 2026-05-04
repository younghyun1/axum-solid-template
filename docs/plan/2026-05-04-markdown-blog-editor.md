# Markdown Blog Editor Plan

Date: 2026-05-04

## Goal

Add a production-grade markdown writing experience for provider and admin blog content while keeping backend rendering safe and deterministic.

## Backend

- [x] Store markdown source as canonical content.
- [x] Render sanitized HTML server-side for public reads.
- [x] Add DTO fields for markdown source, rendered HTML, excerpt, status, moderation state, hero image, and timestamps.
- [x] Validate content length, title length, slug shape, and unsafe HTML.
- Add update, publish/unpublish, archive, and moderation transition APIs.
- [x] Keep rendered output cacheable and indexable.

## Frontend

- [x] Use a modern ProseMirror/markdown editor stack with markdown source as the persisted representation.
- [x] Provide title, slug, excerpt, save/publish actions, and validation states.
- [x] Reuse the editor for provider blog posts and admin central posts.
- [x] Keep editor state typed and isolated from low-level fetch code.

## Acceptance Criteria

- Provider dashboard can create and edit rich markdown posts.
- Admin dashboard can create and edit central blog posts.
- Public detail pages render sanitized blog previews.
- Markdown rendering tests cover links, headings, tables, HTML stripping, and empty content.
