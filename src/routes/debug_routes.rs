use axum::{routing::get, Router};

use crate::{handlers::debug_handler, AppState};

pub fn routes() -> Router<AppState> {
    Router::new().route("/debug/shards", get(debug_handler::shard_stats))
}
