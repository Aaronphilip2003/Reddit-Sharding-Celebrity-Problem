use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::models::responses::post_response::PostResponse;

pub async fn create_post(
    db: &Pool<Postgres>,
    author_id: Uuid,
    title: &str,
    body: &str,
) -> Result<PostResponse, sqlx::Error> {
    sqlx::query_as::<_, PostResponse>(
        "INSERT INTO posts (author_id, title, body) VALUES ($1, $2, $3)
         RETURNING id, author_id, title, body, created_at",
    )
    .bind(author_id)
    .bind(title)
    .bind(body)
    .fetch_one(db)
    .await
}

pub async fn list_posts(db: &Pool<Postgres>) -> Result<Vec<PostResponse>, sqlx::Error> {
    sqlx::query_as::<_, PostResponse>(
        "SELECT id, author_id, title, body, created_at FROM posts ORDER BY created_at DESC",
    )
    .fetch_all(db)
    .await
}
