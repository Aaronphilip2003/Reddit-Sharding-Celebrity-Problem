use sqlx::{Pool, Postgres};

use crate::models::responses::vote_response::VoteResponse;
use crate::repositories::vote_repository;
use crate::sharding;

pub async fn cast_vote(
    shards: &[Pool<Postgres>],
    post_id: i64,
    user_id: i64,
    value: i16,
) -> Result<VoteResponse, sqlx::Error> {
    let shard_index = sharding::shard_for_id(post_id, shards.len() as i64);
    vote_repository::cast_vote(&shards[shard_index], post_id, user_id, value).await
}

pub async fn score_for_post(shards: &[Pool<Postgres>], post_id: i64) -> Result<i64, sqlx::Error> {
    let shard_index = sharding::shard_for_id(post_id, shards.len() as i64);
    vote_repository::score_for_post(&shards[shard_index], post_id).await
}
