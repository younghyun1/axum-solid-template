# Cookie Session Auth

## Goal
- Move browser auth from frontend-held bearer tokens to HttpOnly cookies.
- Add a durable refresh/session cookie so production refreshes preserve auth.
- Make CORS and cookie behavior explicit by deployment environment.

## Tasks
- [x] Add backend cookie/CORS configuration and validation.
- [x] Add refresh-session database migration, domain model, and repository.
- [x] Issue, rotate, revoke, and clear refresh sessions from auth services.
- [x] Update auth middleware to read access JWT cookies.
- [x] Add `/api/v1/auth/refresh` and cookie-aware login/logout responses.
- [x] Update frontend API client and app state to use credentialed requests.
- [x] Update design/API docs.
- [x] Run frontend and backend checks.
- [x] Add dotenv entries for public app URL, allowed origins, and dev cookie security.
