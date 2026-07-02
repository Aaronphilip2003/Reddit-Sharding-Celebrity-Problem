use axum::{extract::State, http::StatusCode, Json};

use crate::{
    models::{requests::post_requests::CreatePostRequest, responses::post_response::PostResponse},
    services::post_service,
    AppState,
};

#[utoipa::path(
    post,
    path = "/posts",
    request_body = CreatePostRequest,
    responses(
        (status = 201, description = "Post created", body = PostResponse)
    )
)]
pub async fn create_post(
    State(state): State<AppState>,
    Json(payload): Json<CreatePostRequest>,
) -> Result<(StatusCode, Json<PostResponse>), StatusCode> {
    post_service::create_post(&state.shards, payload.author_id, &payload.title, &payload.body)
        .await
        .map(|post| (StatusCode::CREATED, Json(post)))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

#[utoipa::path(
    get,
    path = "/posts",
    responses(
        (status = 200, description = "List posts", body = [PostResponse])
    )
)]
pub async fn list_posts(
    State(state): State<AppState>,
) -> Result<Json<Vec<PostResponse>>, StatusCode> {
    post_service::list_posts(&state.shards)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
