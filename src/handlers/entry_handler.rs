use axum::{Json, extract::State};
use crate::models::responses::entry_response::EntryResponse;

use crate::{AppState, services::entry_service};

#[utoipa::path(
    get,
    path = "/entries",
    responses(
        (
            status = 200,
            description = "Returns entries",
            body = [String]
        )
    )
)]

pub async fn get_entries(
    State(state): State<AppState>,
) -> Json<Vec<EntryResponse>> {

    let entries =
        entry_service::get_entries(&state.db)
            .await
            .unwrap();

    Json(entries)
}