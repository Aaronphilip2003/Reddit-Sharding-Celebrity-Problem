use sqlx::{Pool, Postgres};

use crate::models::responses::post_stats_response::PostStatsResponse;
use crate::repositories::{comment_repository, vote_repository};
use crate::sharding;

pub async fn stats_for_post(
    shards: &[Pool<Postgres>],
    post_id: i64,
) -> Result<PostStatsResponse, sqlx::Error> {
    let shard_index = sharding::shard_for_id(post_id, shards.len() as i64);
    let pool = &shards[shard_index];

    let score = vote_repository::score_for_post(pool, post_id).await?;
    let total_votes = vote_repository::count_for_post(pool, post_id).await?;
    let total_comments = comment_repository::count_for_post(pool, post_id).await?;

    Ok(PostStatsResponse {
        post_id,
        score,
        total_votes,
        total_comments,
    })
}
