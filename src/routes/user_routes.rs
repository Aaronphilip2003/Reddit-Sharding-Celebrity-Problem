use axum::{routing::post, Router};

use crate::{handlers::user_handler, AppState};

pub fn routes() -> Router<AppState> {
    Router::new().route("/users", post(user_handler::create_user))
}
