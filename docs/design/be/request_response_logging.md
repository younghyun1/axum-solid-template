# Request And Response Logging

All incoming HTTP requests pass through `be/src/middleware/request_response_logging.rs`.

The middleware logs two structured events per request:

- `HTTP request received` at `INFO`.
- `HTTP response completed` at `INFO`, `WARN` for 4xx responses, and `ERROR` for 5xx responses.

Logged request fields:

- `request_method`
- `request_path`
- `request_query`
- `http_version`
- `user_agent`
- `request_content_type`
- `request_content_length`

Logged response fields:

- `request_method`
- `request_path`
- `request_query`
- `http_version`
- `http_status_code`
- `response_content_type`
- `response_content_length`
- `processing_duration`

`processing_duration` is formatted as an ISO 8601 duration string, matching the API response envelope convention.

The middleware does not read request or response bodies. It only records metadata, status, and elapsed wall-clock time from `Instant`.
