# Error Codes

`be/src/error/code_error.rs` is the canonical API error-code registry.

Rules:

- Add new API errors as `CodeError` constants only in `code_error.rs`.
- Do not reuse numeric codes after an error has been exposed to clients.
- Keep the public `message` stable unless the old message is actively misleading.
- Use the `log_level` to control structured server logging severity.
- Use `ApiError::from_source` for private internal details and `ApiError::public` only when the detail is safe for clients.

Current ranges:

- `1..=49`: backend infrastructure, database, validation, auth, token, and rate-limit errors.
- `50..=199`: feature/domain errors for future modules.
- `200..=254`: integration and external-service errors.
- `255`: generic internal error fallback.

Current auth/security codes:

- `13`: unauthorized request.
- `14`: invalid JWT.
- `15`: email is not verified.
- `16..=18`: email verification token errors.
- `19..=21`: password reset token errors.
- `23`: request was rate-limited.
