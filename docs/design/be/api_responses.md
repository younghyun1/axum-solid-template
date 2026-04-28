# API Response Conventions

All JSON API handlers should return the shared envelope from `be/src/dto/api_response.rs`.

Successful responses:

```json
{
  "success": true,
  "data": {},
  "error": null,
  "meta": {
    "timestamp": "2026-04-28T10:35:12.123Z",
    "processing_duration": "PT0.001042S"
  }
}
```

Error responses:

```json
{
  "success": false,
  "data": null,
  "error": {
    "error_code": 1,
    "error_level": "ERROR",
    "error_message": "Database query failed!"
  },
  "meta": {
    "timestamp": "2026-04-28T10:35:12.123Z",
    "processing_duration": "PT0.001042S",
    "details": {
      "request_id": "018f2f7a-7a39-7d89-a03f-b8b5e1d65021"
    }
  }
}
```

`meta.timestamp` is always a UTC RFC3339 timestamp produced through `chrono`. `meta.processing_duration` is always an ISO 8601 duration string. `meta.details` is optional and accepts any JSON value.

API timing is automatic for routes under `/api/v1`. `be/src/middleware/api_timing.rs` starts an `ApiTimer` before downstream API middleware and handlers run. `ApiMeta::new()` reads that request-scoped timer, so controllers should not start or pass timers manually.

Handlers should use `crate::error::prelude::*` and return `ApiResponseResult<T>` when they can fail:

```rust
pub async fn get_user() -> ApiResponseResult<UserResponse> {
    let user = match repository_call().api_err(CodeError::INTERNAL_ERROR) {
        Ok(user) => user,
        Err(error) => return Err(error),
    };

    Ok(api_ok(user))
}
```

Library errors are logged as private source details by default. Public `error_detail` is omitted unless the handler explicitly uses `api_err_public`, `api_err_with_detail`, `api_ok_or_public`, or `ApiError::public`.

Known API errors are declared as `CodeError` constants in `be/src/error/code_error.rs`. Each constant owns the public message, HTTP status code, internal numeric error code, and tracing log level.
