use axum::{
    routing::{get, post},
    Router,
};

use crate::{handlers::vote_handler, AppState};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/votes", post(vote_handler::cast_vote))
        .route("/posts/{post_id}/score", get(vote_handler::score_for_post))
}
