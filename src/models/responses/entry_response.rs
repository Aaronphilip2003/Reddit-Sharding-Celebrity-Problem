use serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize, FromRow)]
pub struct EntryResponse {
    pub title: String,
    pub content: String,
    pub context: String,
}