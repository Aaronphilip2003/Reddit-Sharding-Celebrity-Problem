use sqlx::{Pool, Postgres};

use crate::models::responses::comment_response::CommentResponse;
use crate::repositories::comment_repository;

pub async fn create_comment(
    db: &Pool<Postgres>,
    post_id: i64,
    author_id: i64,
    body: &str,
) -> Result<CommentResponse, sqlx::Error> {
    comment_repository::create_comment(db, post_id, author_id, body).await
}

pub async fn list_comments_for_post(
    db: &Pool<Postgres>,
    post_id: i64,
) -> Result<Vec<CommentResponse>, sqlx::Error> {
    comment_repository::list_comments_for_post(db, post_id).await
}
