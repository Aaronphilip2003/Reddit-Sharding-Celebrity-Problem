use sqlx::{Pool, Postgres};

use crate::models::responses::post_response::PostResponse;
use crate::repositories::post_repository;
use crate::sharding;

pub async fn create_post(
    shards: &[Pool<Postgres>],
    author_id: i64,
    title: &str,
    body: &str,
) -> Result<PostResponse, sqlx::Error> {
    let shard_index = sharding::shard_for_id(author_id, shards.len() as i64);
    post_repository::create_post(&shards[shard_index], author_id, title, body).await
}

/// Posts live on whichever shard their author does, so listing "all" posts
/// means fanning out to every shard and merging the results.
pub async fn list_posts(shards: &[Pool<Postgres>]) -> Result<Vec<PostResponse>, sqlx::Error> {
    let mut all_posts = Vec::new();

    for shard in shards {
        all_posts.extend(post_repository::list_posts(shard).await?);
    }

    all_posts.sort_by(|a, b| b.created_at.cmp(&a.created_at));

    Ok(all_posts)
}
