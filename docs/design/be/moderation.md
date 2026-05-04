# Backend Moderation Design

Moderation controls are backend-authoritative. Frontend role checks only affect navigation and visibility.

Ban records store target user, actor user, reason, scope, active window, optional revocation fields, and timestamps. Active-ban lookups must account for revoked and expired bans. Authentication/session code should be able to check active bans without expensive joins.

Moderation state should be explicit for provider profiles, provider blog posts, central blog posts, and advertisement banners. Admin and moderator services are responsible for transitions and audit fields.

Advertisement banners are scheduled content with placement, priority, visibility, image, target URL, start/end timestamps, and tracking hooks. Public banner queries should only return visible banners whose schedule window is active.

All moderation actions log structured tracing fields for actor, target, action, and result.
