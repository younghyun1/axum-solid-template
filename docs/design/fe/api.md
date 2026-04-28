# Frontend API Conventions

Frontend API code is centralized under `/fe/src/api`.

- `types.ts` mirrors the backend `ApiEnvelope`, `ApiMeta`, and `ApiErrorBody` response shape.
- `client.ts` is the only low-level fetch wrapper. It validates envelope shape and returns a discriminated `ApiCallResult<T>`.
- Backend error envelopes are normalized into `NormalizedApiError` with `kind`, `message`, `status`, `backendError`, and `meta`.
- `backendApi.ts` owns endpoint metadata and demo request construction.
- `forms.ts` owns reusable demo field definitions and form value parsing.

Components should consume `ApiCallResult<T>` and endpoint metadata rather than hardcoding backend paths or manually parsing error envelopes.
