use axum::{Router, routing::get};
use crate::AppState;

use crate::handlers::hello_handler::hello;

pub fn routes() -> Router<AppState> {
    Router::new().route("/hello", get(hello))
}
