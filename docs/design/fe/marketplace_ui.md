# Marketplace UI Design

The Solid frontend presents the marketplace as the default product surface while preserving cookie-based auth.

Primary route groups:

- public directory and provider detail pages
- user account, profile, and transaction views
- provider dashboard for profile, media, blog, and payment state
- admin dashboard for users, bans, moderation, central blog, banners, and overview

TypeScript remains strict. API envelope types and new marketplace calls belong under `fe/src/api`. Pages should consume typed DTOs and avoid duplicating backend response shapes.

Styling stays centralized under `fe/src/styles`. Components should use semantic classes and shared layout primitives instead of ad hoc utility strings. Marketplace pages should be information-dense, calm, and operational, with stable dimensions for cards, image slots, tables, and controls.

Role-specific navigation is derived from the authenticated profile but backend authorization remains authoritative. Public pages must work without an authenticated session.

Provider cards should show stable image placeholders while media is pending. Admin tables should support scanning and repeated actions without large marketing-style hero sections.
