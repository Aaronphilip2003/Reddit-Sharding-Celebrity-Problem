use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, ToSchema)]
pub struct CreatePostRequest {
    pub author_id: Uuid,
    pub title: String,
    pub body: String,
}
