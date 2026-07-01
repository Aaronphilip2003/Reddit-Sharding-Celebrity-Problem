use sqlx::{Pool, Postgres};

use crate::models::responses::user_response::UserResponse;
use crate::repositories::user_repository;

pub async fn create_user(db: &Pool<Postgres>, username: &str) -> Result<UserResponse, sqlx::Error> {
    user_repository::create_user(db, username).await
}
