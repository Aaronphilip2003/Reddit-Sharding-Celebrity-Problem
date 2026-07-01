use sqlx::{Pool, Postgres};

use crate::models::responses::vote_response::VoteResponse;
use crate::repositories::vote_repository;

pub async fn cast_vote(
    db: &Pool<Postgres>,
    post_id: i64,
    user_id: i64,
    value: i16,
) -> Result<VoteResponse, sqlx::Error> {
    vote_repository::cast_vote(db, post_id, user_id, value).await
}

pub async fn score_for_post(db: &Pool<Postgres>, post_id: i64) -> Result<i64, sqlx::Error> {
    vote_repository::score_for_post(db, post_id).await
}
