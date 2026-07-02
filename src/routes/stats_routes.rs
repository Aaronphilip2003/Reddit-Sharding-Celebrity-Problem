use axum::{routing::get, Router};

use crate::{handlers::stats_handler, AppState};

pub fn routes() -> Router<AppState> {
    Router::new().route("/posts/{post_id}/stats", get(stats_handler::stats_for_post))
}
