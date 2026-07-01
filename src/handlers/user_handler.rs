use axum::{extract::State, http::StatusCode, Json};

use crate::{
    models::{requests::user_requests::CreateUserRequest, responses::user_response::UserResponse},
    services::user_service,
    AppState,
};

#[utoipa::path(
    post,
    path = "/users",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User created", body = UserResponse)
    )
)]
pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<UserResponse>), StatusCode> {
    user_service::create_user(&state.db, &payload.username)
        .await
        .map(|user| (StatusCode::CREATED, Json(user)))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
