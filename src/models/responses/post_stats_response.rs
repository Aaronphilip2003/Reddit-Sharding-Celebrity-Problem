use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct PostStatsResponse {
    pub post_id: i64,
    pub score: i64,
    pub total_votes: i64,
    pub total_comments: i64,
}
