use axum::{
    routing::{get, post},
    Router,
};

use crate::{handlers::comment_handler, AppState};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/comments", post(comment_handler::create_comment))
        .route(
            "/posts/{post_id}/comments",
            get(comment_handler::list_comments_for_post),
        )
}
