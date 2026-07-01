use sqlx::{Pool, Postgres};

use crate::models::responses::user_response::UserResponse;

pub async fn create_user(
    db: &Pool<Postgres>,
    username: &str,
) -> Result<UserResponse, sqlx::Error> {
    sqlx::query_as::<_, UserResponse>(
        "INSERT INTO users (username) VALUES ($1) RETURNING id, username, created_at",
    )
    .bind(username)
    .fetch_one(db)
    .await
}
