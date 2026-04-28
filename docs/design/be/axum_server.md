# Axum Server Conventions

The backend exposes a single Axum binary from `be/src/main.rs`.

- Runtime initialization stays in `be/src/init`.
- The long-running server future is spawned from `main` so it runs as a Tokio worker task rather than the `#[tokio::main]` root future.
- Route composition stays in `be/src/router`.
- JSON API routes are exposed only under `/api/v1`.
- Request handlers stay in `be/src/controller`.
- API documentation is generated with `utoipa` and served through `/api/v1/swagger-ui` and `/api/v1/api-docs/openapi.json`.
- Static frontend assets are deployed into `/be/fe`, embedded with `rust_embed`, loaded into an in-memory asset map, and served as the fallback route.
- Precompressed `.zst` assets are preferred over `.gz` when the client advertises compatible `Accept-Encoding`; identity assets remain available for clients that do not advertise either encoding.
- HTTPS uses rustls through `axum_server::Server::from_listener` with a `tokio::net::TcpListener`. When HTTPS is enabled, a separate HTTP listener redirects traffic to HTTPS.
- Compression is centralized at the router boundary with gzip and zstd enabled.
- Request and response logging is centralized at the router boundary in `be/src/middleware/request_response_logging.rs`.
- CORS is permissive only for `local` and `development`; production modes use restrictive CORS defaults.

Environment keys:

- `SERVER_BIND_IP`: IP address for the main listener. Defaults to `127.0.0.1`.
- `SERVER_PORT`: port for the main listener. Defaults to `3000`.
- `HTTP_REDIRECT_PORT`: HTTP redirect listener port when HTTPS is enabled. Defaults to `8080`.
