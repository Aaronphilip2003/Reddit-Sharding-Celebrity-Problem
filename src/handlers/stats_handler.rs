use axum::{extract::Path, extract::State, http::StatusCode, Json};

use crate::{
    models::responses::post_stats_response::PostStatsResponse, services::stats_service, AppState,
};

#[utoipa::path(
    get,
    path = "/posts/{post_id}/stats",
    params(("post_id" = i64, Path, description = "Post id")),
    responses(
        (status = 200, description = "Live vote/comment stats for a post", body = PostStatsResponse)
    )
)]
pub async fn stats_for_post(
    State(state): State<AppState>,
    Path(post_id): Path<i64>,
) -> Result<Json<PostStatsResponse>, StatusCode> {
    stats_service::stats_for_post(&state.shards, post_id)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
