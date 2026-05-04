# Database Reset Auth State

## Goal
- Explain where local JWT auth state lives during frontend testing.
- Verify why a user can appear logged in after database reset.
- Fix the reset flow or auth validation so reset does not leave stale logged-in state.

## Tasks
- [x] Inspect frontend token/session storage.
- [x] Inspect backend auth middleware and `/auth/me` behavior.
- [x] Decide whether reset should clear frontend session, invalidate backend JWTs, or both.
- [x] Implement the scoped fix.
- [x] Run relevant frontend/backend checks.
