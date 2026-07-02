use sqlx::{Pool, Postgres};

use crate::models::responses::comment_response::CommentResponse;
use crate::repositories::comment_repository;
use crate::sharding;

pub async fn create_comment(
    shards: &[Pool<Postgres>],
    post_id: i64,
    author_id: i64,
    body: &str,
) -> Result<CommentResponse, sqlx::Error> {
    let shard_index = sharding::shard_for_id(post_id, shards.len() as i64);
    comment_repository::create_comment(&shards[shard_index], post_id, author_id, body).await
}

pub async fn list_comments_for_post(
    shards: &[Pool<Postgres>],
    post_id: i64,
) -> Result<Vec<CommentResponse>, sqlx::Error> {
    let shard_index = sharding::shard_for_id(post_id, shards.len() as i64);
    comment_repository::list_comments_for_post(&shards[shard_index], post_id).await
}
