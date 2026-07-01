use axum::{extract::Path, extract::State, http::StatusCode, Json};

use crate::{
    models::{
        requests::comment_requests::CreateCommentRequest,
        responses::comment_response::CommentResponse,
    },
    services::comment_service,
    AppState,
};

#[utoipa::path(
    post,
    path = "/comments",
    request_body = CreateCommentRequest,
    responses(
        (status = 201, description = "Comment created", body = CommentResponse)
    )
)]
pub async fn create_comment(
    State(state): State<AppState>,
    Json(payload): Json<CreateCommentRequest>,
) -> Result<(StatusCode, Json<CommentResponse>), StatusCode> {
    comment_service::create_comment(&state.db, payload.post_id, payload.author_id, &payload.body)
        .await
        .map(|comment| (StatusCode::CREATED, Json(comment)))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

#[utoipa::path(
    get,
    path = "/posts/{post_id}/comments",
    params(("post_id" = i64, Path, description = "Post id")),
    responses(
        (status = 200, description = "List comments for a post", body = [CommentResponse])
    )
)]
pub async fn list_comments_for_post(
    State(state): State<AppState>,
    Path(post_id): Path<i64>,
) -> Result<Json<Vec<CommentResponse>>, StatusCode> {
    comment_service::list_comments_for_post(&state.db, post_id)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
