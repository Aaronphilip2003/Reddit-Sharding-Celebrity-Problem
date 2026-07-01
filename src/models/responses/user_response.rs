use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Serialize, FromRow, ToSchema)]
pub struct UserResponse {
    pub id: i64,
    pub username: String,
    pub created_at: DateTime<Utc>,
}
