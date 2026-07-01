use sqlx::{Pool, Postgres};

use crate::models::responses::comment_response::CommentResponse;

pub async fn create_comment(
    db: &Pool<Postgres>,
    post_id: i64,
    author_id: i64,
    body: &str,
) -> Result<CommentResponse, sqlx::Error> {
    sqlx::query_as::<_, CommentResponse>(
        "INSERT INTO comments (post_id, author_id, body) VALUES ($1, $2, $3)
         RETURNING id, post_id, author_id, body, created_at",
    )
    .bind(post_id)
    .bind(author_id)
    .bind(body)
    .fetch_one(db)
    .await
}

pub async fn list_comments_for_post(
    db: &Pool<Postgres>,
    post_id: i64,
) -> Result<Vec<CommentResponse>, sqlx::Error> {
    sqlx::query_as::<_, CommentResponse>(
        "SELECT id, post_id, author_id, body, created_at FROM comments
         WHERE post_id = $1 ORDER BY created_at ASC",
    )
    .bind(post_id)
    .fetch_all(db)
    .await
}
