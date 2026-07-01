use axum::{
    routing::get,
    Router,
};

use crate::{
    handlers::entry_handler,
    AppState,
};

pub fn routes() -> Router<AppState> {

    Router::new()
        .route(
            "/entries",
            get(entry_handler::get_entries),
        )
}