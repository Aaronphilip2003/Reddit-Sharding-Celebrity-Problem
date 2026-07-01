use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, FromRow, ToSchema)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
    pub created_at: DateTime<Utc>,
}
