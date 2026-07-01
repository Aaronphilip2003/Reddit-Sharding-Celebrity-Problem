use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct CastVoteRequest {
    pub post_id: i64,
    pub user_id: i64,
    /// Must be 1 (upvote) or -1 (downvote).
    pub value: i16,
}
