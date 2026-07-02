use axum::{extract::Path, extract::State, http::StatusCode, Json};

use crate::{
    models::{
        requests::vote_requests::CastVoteRequest,
        responses::vote_response::{ScoreResponse, VoteResponse},
    },
    services::vote_service,
    AppState,
};

#[utoipa::path(
    post,
    path = "/votes",
    request_body = CastVoteRequest,
    responses(
        (status = 201, description = "Vote recorded (or updated if the user already voted)", body = VoteResponse)
    )
)]
pub async fn cast_vote(
    State(state): State<AppState>,
    Json(payload): Json<CastVoteRequest>,
) -> Result<(StatusCode, Json<VoteResponse>), StatusCode> {
    vote_service::cast_vote(&state.shards, payload.post_id, payload.user_id, payload.value)
        .await
        .map(|vote| (StatusCode::CREATED, Json(vote)))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

#[utoipa::path(
    get,
    path = "/posts/{post_id}/score",
    params(("post_id" = i64, Path, description = "Post id")),
    responses(
        (status = 200, description = "Net vote score for a post", body = ScoreResponse)
    )
)]
pub async fn score_for_post(
    State(state): State<AppState>,
    Path(post_id): Path<i64>,
) -> Result<Json<ScoreResponse>, StatusCode> {
    vote_service::score_for_post(&state.shards, post_id)
        .await
        .map(|score| Json(ScoreResponse { post_id, score }))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
