mod api_docs;
mod handlers;
mod models;
mod repositories;
mod routes;
mod services;

use api_docs::ApiDoc;
use axum::Router;
use sqlx::{Pool, Postgres};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{EnvFilter, fmt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
mod config;

use config::database::create_pool;

#[derive(Clone)]
pub struct AppState {
    pub db: Pool<Postgres>,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let pool = create_pool().await;

    let app_state = AppState { db: pool };

    fmt()
        .with_env_filter(EnvFilter::new("hello_api=info,tower_http=debug"))
        .init();

    let app = Router::new()
        .merge(routes::hello_routes::routes())
        .merge(routes::user_routes::routes())
        .merge(routes::post_routes::routes())
        .merge(routes::comment_routes::routes())
        .merge(routes::vote_routes::routes())
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .with_state(app_state)
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();

    println!("Server running on http://localhost:8000");
    println!("Swagger UI at http://localhost:8000/swagger-ui");
    println!("Connected to postgres");

    axum::serve(listener, app).await.unwrap();
}
