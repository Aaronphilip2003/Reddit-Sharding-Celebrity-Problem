use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, FromRow, ToSchema)]
pub struct PostResponse {
    pub id: Uuid,
    pub author_id: Uuid,
    pub title: String,
    pub body: String,
    pub created_at: DateTime<Utc>,
}
