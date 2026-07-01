# Axum API Template (Rust)

Opinionated starter template for a small-but-real Rust API using:

- `axum` for HTTP routing
- `utoipa` + `utoipa-swagger-ui` for OpenAPI + Swagger UI
- `tracing` + `tower-http` for request logging

## Quick Start

Run the server:

```bash
cargo run
```

Then open:

- API: `http://localhost:8000/hello`
- Swagger UI: `http://localhost:8000/swagger-ui`
- OpenAPI JSON: `http://localhost:8000/api-docs/openapi.json`

## Project Structure

This template keeps the responsibilities separated so your codebase scales cleanly:

```text
src/
├── main.rs               # app wiring (router, middleware, server)
├── api_docs.rs           # OpenAPI registry (paths + schemas)
├── routes/               # route definitions only (no business logic)
├── handlers/             # HTTP layer: request -> service -> response
├── models/               # request/response DTOs (serde + ToSchema)
└── services/             # business logic (add when needed)
└── config/               # database configuration
└── repositories/         # actual sql logic

```

## Adding A New Endpoint

Typical flow:

1. Add a response/request model (if needed) in `src/models/...`
2. Add a handler function in `src/handlers/...` and annotate it with `#[utoipa::path(...)]`
3. Add the route in `src/routes/...`
4. Register the handler + schemas in `src/api_docs.rs`

## Logging

By default the server initializes `tracing_subscriber` in `src/main.rs`.

If you want to override logging at runtime:

```bash
RUST_LOG="hello_api=info,tower_http=debug" cargo run
```
