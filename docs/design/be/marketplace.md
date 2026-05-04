# Backend Marketplace Design

The marketplace backend is split by role:

- public users browse published provider profiles and public provider blog posts
- authenticated users maintain their own profile extension and create payment intents
- service providers maintain their provider profile, images, blog posts, and payment views
- admins and moderators manage bans, moderation state, central blog posts, banners, and oversight

Domain objects live in `be/src/domain`. API request and response DTOs live in `be/src/dto` and derive `utoipa::ToSchema` whenever exposed. PostgreSQL repositories live under `be/src/repository/**/postgres`. Workflows and authorization live in `be/src/service`. Controllers live under `be/src/controller/v1`.

Public directory queries must be built to avoid N+1 behavior. Directory cards should fetch provider summary, primary image metadata, and public aggregate fields in one query or bounded batched queries.

Provider slugs are unique, stable public identifiers. Slug generation must handle collisions explicitly and should be fuzz-tested.

Moderation and publication are separate states. A provider can draft content, but public reads only return content that is published and allowed by moderation.
