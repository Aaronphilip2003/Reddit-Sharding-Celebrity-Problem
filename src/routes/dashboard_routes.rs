use axum::{routing::get, Router};

use crate::{handlers::dashboard_handler, AppState};

pub fn routes() -> Router<AppState> {
    Router::new().route("/dashboard", get(dashboard_handler::dashboard))
}
