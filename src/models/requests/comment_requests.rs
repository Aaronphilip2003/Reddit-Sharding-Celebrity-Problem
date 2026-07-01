use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct CreateCommentRequest {
    pub post_id: i64,
    pub author_id: i64,
    pub body: String,
}
