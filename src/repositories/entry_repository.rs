use crate::models::responses::entry_response::EntryResponse;
use sqlx::{Pool, Postgres};

pub async fn get_entries(
    db: &Pool<Postgres>,
) -> Result<Vec<EntryResponse>, sqlx::Error> {

    let entries =
        sqlx::query_as::<_, EntryResponse>(
            "SELECT title, content, context FROM public.entries"
        )
        .fetch_all(db)
        .await?;

    Ok(entries)
}