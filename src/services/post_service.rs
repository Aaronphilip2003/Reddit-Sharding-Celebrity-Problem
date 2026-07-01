use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::models::responses::post_response::PostResponse;
use crate::repositories::post_repository;

pub async fn create_post(
    db: &Pool<Postgres>,
    author_id: Uuid,
    title: &str,
    body: &str,
) -> Result<PostResponse, sqlx::Error> {
    post_repository::create_post(db, author_id, title, body).await
}

pub async fn list_posts(db: &Pool<Postgres>) -> Result<Vec<PostResponse>, sqlx::Error> {
    post_repository::list_posts(db).await
}
