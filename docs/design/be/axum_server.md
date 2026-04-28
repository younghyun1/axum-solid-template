# Axum Server Conventions

The backend exposes a single Axum binary from `be/src/main.rs`.

- Runtime initialization stays in `be/src/init`.
- Route composition stays in `be/src/router`.
- Request handlers stay in `be/src/controller`.
- API documentation is generated with `utoipa` and served through Swagger UI.
- Static frontend assets are embedded from `/fe` and served as the fallback route.
- HTTPS uses rustls. When HTTPS is enabled, a separate HTTP listener redirects traffic to HTTPS.
- Compression is centralized at the router boundary with gzip and zstd enabled.
- Request and response logging is centralized at the router boundary in `be/src/middleware/request_response_logging.rs`.
- CORS is permissive only for `local` and `development`; production modes use restrictive CORS defaults.

Environment keys:

- `SERVER_BIND_IP`: IP address for the main listener. Defaults to `127.0.0.1`.
- `SERVER_PORT`: port for the main listener. Defaults to `3000`.
- `HTTP_REDIRECT_PORT`: HTTP redirect listener port when HTTPS is enabled. Defaults to `8080`.
