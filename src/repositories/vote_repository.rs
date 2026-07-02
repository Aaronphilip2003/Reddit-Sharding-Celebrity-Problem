use sqlx::{Pool, Postgres};

use crate::models::responses::vote_response::VoteResponse;

pub async fn cast_vote(
    db: &Pool<Postgres>,
    post_id: i64,
    user_id: i64,
    value: i16,
) -> Result<VoteResponse, sqlx::Error> {
    sqlx::query_as::<_, VoteResponse>(
        "INSERT INTO votes (post_id, user_id, value) VALUES ($1, $2, $3)
         ON CONFLICT (post_id, user_id) DO UPDATE SET value = EXCLUDED.value
         RETURNING id, post_id, user_id, value, created_at",
    )
    .bind(post_id)
    .bind(user_id)
    .bind(value)
    .fetch_one(db)
    .await
}

pub async fn score_for_post(db: &Pool<Postgres>, post_id: i64) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar::<_, i64>(
        "SELECT COALESCE(SUM(value), 0)::bigint FROM votes WHERE post_id = $1",
    )
    .bind(post_id)
    .fetch_one(db)
    .await
}

pub async fn count_for_post(db: &Pool<Postgres>, post_id: i64) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM votes WHERE post_id = $1")
        .bind(post_id)
        .fetch_one(db)
        .await
}
