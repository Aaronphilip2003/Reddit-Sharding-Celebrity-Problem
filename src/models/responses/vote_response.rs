use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Serialize, FromRow, ToSchema)]
pub struct VoteResponse {
    pub id: i64,
    pub post_id: i64,
    pub user_id: i64,
    pub value: i16,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, ToSchema)]
pub struct ScoreResponse {
    pub post_id: i64,
    pub score: i64,
}
