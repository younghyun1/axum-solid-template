# Marketplace Frontend Production Layout Plan

Date: 2026-05-04

## Goal

Replace the first-pass marketplace UI with a credible production listing experience. The public provider directory and provider detail pages should feel like an actual marketplace product surface, not a scaffold.

## Scope

- Public provider directory route.
- Public provider detail route.
- Shared marketplace CSS and responsive layout primitives.
- Navigation affordances needed to make the public marketplace the default product surface.

Provider dashboard, user account, admin dashboard, markdown blog editing, Tantivy search, and hybrid cache work are separate follow-up slices.

## Design Direction

- Listing-site density: structured filter rail, result count, sort controls, compact metadata, and stable image slots.
- Trust-first cards: provider status, service area, profile headline, publication/moderation state when relevant, and direct detail navigation.
- Provider detail pages should have a clear service overview, image/media band, provider facts, blog preview, and payment/contact entry point.
- No oversized marketing hero or decorative gradient treatment.
- No ad hoc one-off utility styling inside page components; reusable classes live under `fe/src/styles`.

## Execution Order

- [x] Persist this plan and update the master plan.
- [x] Inspect current `ProviderDirectoryPage`, `ProviderDetailPage`, navigation, marketplace CSS, and API DTOs.
- [x] Refactor page structure only where it improves production UX or file size.
- [x] Add reusable marketplace layout classes for toolbar, filters, cards, detail shell, facts, and states.
- [x] Verify the app with `npm run build` and `npm test`.
- [x] Commit with a concise frontend-focused message.

## Acceptance Criteria

- Directory page looks intentional with no sparse scaffold feel.
- Filters and search controls are visually production-grade even while backend filtering remains query-string based.
- Provider cards have stable dimensions and preserve layout with missing images.
- Detail page presents a provider like a real listing profile with obvious next actions.
- Strict TypeScript build passes.
- Changed files remain below the 300 LOC convention where practical.
