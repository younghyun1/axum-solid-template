# Frontend API Conventions

Frontend API code is centralized under `/fe/src/api`.

- `types.ts` mirrors the backend `ApiEnvelope`, `ApiMeta`, and `ApiErrorBody` response shape.
- `client.ts` is the only low-level fetch wrapper. It validates envelope shape and returns a discriminated `ApiCallResult<T>`.
- Backend error envelopes are normalized into `NormalizedApiError` with `kind`, `message`, `status`, `backendError`, and `meta`.
- `appApi.ts` owns customer-facing typed API calls for health, auth, recovery, and reference-data flows.

Components should consume `ApiCallResult<T>` through `appApi.ts` rather than hardcoding backend paths or manually parsing error envelopes.

Reference data is loaded through:

- `GET /api/v1/reference/countries`
- `GET /api/v1/reference/languages`
- `GET /api/v1/reference/countries/{country_code}/subdivisions`

Country labels should display `country_flag`. Subdivision labels should display `country_flag` from the subdivision response. Signup language options should put the selected country's `country_primary_language` first.
