use sqlx::{Pool, Postgres};
use crate::models::responses::entry_response::EntryResponse;

use crate::repositories::entry_repository;

pub async fn get_entries(
    db: &Pool<Postgres>,
) -> Result<Vec<EntryResponse>, sqlx::Error> {
    entry_repository::get_entries(db).await
}