# Email Verification Bot Deterrence Plan

## Summary

Email verification should use a styled frontend page instead of a raw token display. The page
issues a backend challenge, solves proof-of-work in the browser, asks one local question, and
submits the result before the backend marks the email verification token used.

The system is single-server and on-prem: challenges and questionnaire data are persisted in
PostgreSQL through Diesel migrations and mirrored in RAM. No Redis and no third-party CAPTCHA
provider are used.

## Backend Changes

- Add a Diesel migration for `email_verification_questions`,
  `email_verification_question_answers`, `email_verification_questionnaire_state`, and
  `email_verification_challenges`.
- Seed the initial English question set and accepted answers in the migration itself.
- Store original answer text plus normalized answer text. Matching is case-insensitive by trimming,
  collapsing whitespace, and lowercasing user input before comparison.
- Add public APIs:
  - `GET /api/v1/auth/email-verification/challenge`
  - `POST /api/v1/auth/verify-user-email`
- Add admin-only questionnaire APIs for listing, creating, and deleting questions and answers.
- Add an in-memory challenge/questionnaire cache on `ServerState`. Admin writes persist to DB first,
  bump questionnaire revision, then refresh RAM before returning.
- Verification submit validates rate limits, challenge expiry, honeypot, minimum elapsed time,
  proof-of-work nonce, case-insensitive answer, and token state before marking the token used.

## Frontend Changes

- Verification emails link to `/verify-email?email_validation_token_id=<uuid>`.
- Add a `VerifyEmailPage` that loads a challenge, solves proof-of-work in a Web Worker, shows a
  spinner, asks the local question, and shows verified/authenticated status after submit.
- Add an admin-only top-bar entry for verification challenges, visible only to admin users.
- Add an admin questionnaire page for question creation, answer creation, answer deletion, and
  question deletion.
- Keep all new API calls strictly typed under `fe/src/api`.

## Tests And Checks

- Backend unit tests cover answer normalization and leading-zero proof checks.
- Frontend production build must pass `npm run build`.
- Backend checks must pass `cargo fmt`, `cargo check`, and `cargo clippy`.
- Manual flow: signup email link -> verification page -> proof spinner -> answer -> verified state.
- Manual admin flow: sign in as admin -> top-bar verification page -> create/delete question and
  answer -> refresh shows synchronized revision.

## Implementation Status

- [x] Diesel migration creates and seeds verification questionnaire tables.
- [x] Backend challenge issue and submit APIs are implemented and documented in utoipa.
- [x] Questionnaire cache loads from DB, serves reads from RAM, and refreshes after admin writes.
- [x] Case-insensitive answer normalization is implemented and unit tested.
- [x] Verification email template links to the frontend verification page.
- [x] Frontend verification page solves proof-of-work in a worker and waits for minimum elapsed time.
- [x] Admin-only frontend page supports question and answer creation/deletion.
- [x] Source files have been modularized under the 300 LOC target.

## Assumptions

- Email verification does not auto-login the user. It refreshes `/auth/me` only if a JWT is already
  present.
- Cache synchronization is perfect for supported writes through the backend admin API. Manual DB
  edits require restart or a future database notification mechanism.
- The initial questionnaire is English-only.
