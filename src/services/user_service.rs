use sqlx::{Pool, Postgres};

use crate::models::responses::user_response::UserResponse;
use crate::repositories::user_repository;
use crate::sharding;

pub async fn create_user(
    shards: &[Pool<Postgres>],
    username: &str,
) -> Result<UserResponse, sqlx::Error> {
    let shard_index = sharding::shard_for_username(username, shards.len() as i64);
    user_repository::create_user(&shards[shard_index], username).await
}
