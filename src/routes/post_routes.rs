use axum::{routing::get, Router};

use crate::{handlers::post_handler, AppState};

pub fn routes() -> Router<AppState> {
    Router::new().route(
        "/posts",
        get(post_handler::list_posts).post(post_handler::create_post),
    )
}
