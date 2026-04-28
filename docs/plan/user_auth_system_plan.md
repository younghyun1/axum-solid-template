# User And Auth System Plan

This plan covers the next user/auth changes after adding public signup for `user` and
`service_provider`.

## Current State

- Email verification already exists for all users through `email_verification_tokens` and
  `users.user_is_email_verified`.
- Public signup can create either `user` or `service_provider`.
- Roles are persisted in `roles` and assigned through `user_roles`, but role rank is currently
  hardcoded in `be/src/domain/auth/role.rs`.
- `permissions` and `role_permissions` exist in the DB, but backend authorization currently uses
  role helper methods and rank checks, not persisted permission rows.
- Admin users have frontend access to Swagger, but there is no admin interface for user, role, or
  service-provider operations.

## Service Provider Verification

Service providers need a second verification state beyond email verification. Email verification
proves address ownership. Provider verification proves the account is allowed to offer services.

Recommended model:

- Keep `service_provider` as the signup account type.
- Add a provider verification status that gates provider-only actions.
- Do not treat `service_provider` role alone as permission to provide services.

Suggested DB additions:

- `service_provider_verification_status` enum:
  - `draft`
  - `pending_review`
  - `needs_information`
  - `approved`
  - `rejected`
  - `suspended`
- `service_provider_profiles`
  - `service_provider_profile_id UUID PRIMARY KEY DEFAULT uuidv7()`
  - `user_id UUID NOT NULL UNIQUE REFERENCES users(user_id) ON DELETE CASCADE`
  - `service_provider_status service_provider_verification_status NOT NULL`
  - `service_provider_display_name TEXT NOT NULL`
  - `service_provider_description TEXT`
  - `service_provider_submitted_at TIMESTAMPTZ`
  - `service_provider_reviewed_by UUID REFERENCES users(user_id)`
  - `service_provider_reviewed_at TIMESTAMPTZ`
  - `service_provider_review_note TEXT`
  - `service_provider_created_at TIMESTAMPTZ NOT NULL DEFAULT now()`
  - `service_provider_updated_at TIMESTAMPTZ NOT NULL DEFAULT now()`
- `service_provider_verification_events`
  - append-only audit trail for status transitions and admin notes.
- Optional later table: `service_provider_documents`
  - only add once file storage, retention, access logging, and sensitive-data handling are defined.

Signup behavior:

- User signup creates only a `users` row, `user_roles` row, and email verification token.
- Service-provider signup creates the same auth records plus a `service_provider_profiles` row in
  `draft` or `pending_review`.
- The FE should show provider verification status after signup and login.

Backend behavior:

- Add provider profile DTOs under `be/src/dto`.
- Add domain structs under `be/src/domain`.
- Add Diesel migration and repository functions for provider profile/status reads and updates.
- Include provider verification status in `/api/v1/auth/me` or expose a dedicated
  `/api/v1/service-provider/me` endpoint.
- Gate provider service actions with both:
  - role is exactly `service_provider`
  - provider status is `approved`
- When status changes, increment `users.user_auth_token_version` or otherwise make stale JWTs unable
  to continue using old provider capabilities.
- Document all endpoints in `utoipa`.

Frontend behavior:

- Signup should keep the existing `User` and `Service provider` account-type choice.
- Service providers should land on a provider onboarding/status panel after signup/login.
- Provider status states should be explicit:
  - draft: complete profile
  - pending_review: submitted and waiting
  - needs_information: admin requested changes
  - approved: provider tools enabled
  - rejected/suspended: provider tools disabled with admin note where appropriate
- Provider-only navigation should remain hidden or disabled until status is `approved`.

## Admin And Moderator Management

Admins need an interface to add admins and moderators. This should be a first-class admin surface,
not a public signup path.

Recommended BE endpoints:

- `GET /api/v1/admin/users`
  - filter by role, email, username, email verification, provider status.
- `GET /api/v1/admin/users/{user_id}`
- `PATCH /api/v1/admin/users/{user_id}/role`
  - admin-only.
  - allow assigning `admin`, `moderator`, `service_provider`, or `user`.
  - reject demoting the last admin.
  - reject self-demotion unless another admin confirms through a separate flow.
- `POST /api/v1/admin/staff-invitations`
  - admin invites an email as `admin` or `moderator`.
  - safer than directly creating staff accounts with passwords.
- `PATCH /api/v1/admin/service-providers/{user_id}/verification`
  - admin or moderator can review provider applications if policy allows moderators to review.
  - status transitions should be validated.

Suggested DB additions:

- `admin_audit_events`
  - `admin_audit_event_id UUID PRIMARY KEY DEFAULT uuidv7()`
  - `actor_user_id UUID NOT NULL REFERENCES users(user_id)`
  - `target_user_id UUID REFERENCES users(user_id)`
  - `admin_audit_action TEXT NOT NULL`
  - `admin_audit_metadata JSONB NOT NULL DEFAULT '{}'`
  - `admin_audit_created_at TIMESTAMPTZ NOT NULL DEFAULT now()`
- Optional `staff_invitations`
  - invite token, target email, target role, expiry, accepted time, revoked time.

Frontend admin interface:

- Add an admin route or page behind the signed-in admin gate.
- Tabs:
  - Users
  - Provider verification
  - Staff access
  - Audit
- Staff access should support inviting admins/moderators, revoking pending invites, and changing
  existing staff roles.
- Provider verification should support approve, reject, needs-information, suspend, and notes.

## Role And Action Gating Model

I partially agree with replacing the current junction-table permission model, depending on the
target authorization model.

If authorization is strictly hierarchical, `role_permissions` is unnecessary. Store role rank in the
DB and let each protected action declare the minimum role. That matches the existing Rust
`has_min_role` design and avoids N+1 permission lookups.

Recommended hierarchical schema:

- Add to `roles`:
  - `role_rank SMALLINT NOT NULL UNIQUE`
- Add or repurpose `permissions` as action policy rows:
  - `permission_key TEXT NOT NULL UNIQUE`
  - `permission_description TEXT`
  - `permission_min_role_id UUID NOT NULL REFERENCES roles(role_id)`
  - `permission_requires_email_verified BOOLEAN NOT NULL DEFAULT true`
  - `permission_requires_service_provider_approved BOOLEAN NOT NULL DEFAULT false`
- Drop `role_permissions` after migration if there are no non-hierarchical permissions.

I would not put "minimum rank" on the `roles` table itself. The minimum role/rank belongs to the
action or permission being guarded. The `roles` table should own each role's own rank.

Keep `role_permissions` only if the app needs non-monotonic permissions, for example:

- a `service_provider` can manage provider availability, but a `moderator` should not automatically
  be able to do that despite a higher rank.
- a role needs one-off capabilities that do not follow the rank hierarchy.

If those exceptions are likely, use a hybrid model:

- `roles.role_rank` for coarse hierarchy.
- `permissions.permission_min_role_id` for monotonic gates.
- optional `role_permissions` only for explicit grants or denials.

Given the current codebase, the simplest near-term path is:

1. Add `roles.role_rank` and seed canonical ranks.
2. Update `RoleType` tests to assert DB seed values match Rust ranks.
3. Leave `permissions` and `role_permissions` unused until a real non-hierarchical permission is
   needed, or migrate `permissions` into action policies and remove `role_permissions`.

## Other User/Auth Observations

- Role and provider-status changes need token invalidation. `user_auth_token_version` already exists,
  but protected middleware should enforce it if role/status changes must take effect immediately.
- JWT claims include role metadata. That is convenient, but DB-backed checks should be used for
  sensitive mutable state such as provider approval and suspension.
- Signup currently checks email existence before insert. Keep the unique index as the source of
  truth; the pre-check is only UX.
- Admin role assignment should be audited before it is exposed in the FE.
- There should be a documented first-admin creation path as a Rust binary in the backend workspace.
- If mail is disabled, signup works but verification UX is incomplete. Local development can log
  tokens, but production should require a working mail provider before public signup is enabled.
- Provider verification may eventually require uploaded documents. Do not add document upload until
  file retention, access control, audit logging, and deletion semantics are defined.
- Consider separating "account role" from "capability status". For example, a user may have
  `service_provider` role but `pending_review` status; provider actions still remain blocked.
- Add admin and provider endpoints to Swagger as soon as their DTOs exist so FE integration remains
  typed against documented response shapes.
