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

## Public Listing Layout

The public provider directory should follow mature listing-site conventions:

- a compact page header with result context and primary search
- a persistent filter/sort area on desktop and a stacked control area on mobile
- dense result cards with image, service area, headline, provider status, and clear detail navigation
- empty, loading, and error states that preserve page geometry
- stable card and media dimensions so async data does not shift the page

The directory should not use a marketing hero, decorative gradient background, or large explanatory copy. Public browsing is the core task.

Provider detail pages should read as operational profiles. They should put the provider name, headline, service area, media, trust facts, blog preview, and payment/contact action in the first meaningful viewport. Long-form copy belongs in structured sections rather than a single sparse panel.
