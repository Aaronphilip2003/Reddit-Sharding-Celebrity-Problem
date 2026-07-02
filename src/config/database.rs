use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::env;

/// Connects to one pool per shard, discovered from SHARD_0_DATABASE_URL,
/// SHARD_1_DATABASE_URL, ... until the next index isn't set. Each pool is
/// capped low so a hot shard's pool visibly saturates instead of masking
/// the imbalance behind a huge connection budget.
pub async fn create_shard_pools() -> Vec<Pool<Postgres>> {
    let mut shards = Vec::new();
    let mut index = 0;

    loop {
        let key = format!("SHARD_{index}_DATABASE_URL");
        let Ok(database_url) = env::var(&key) else {
            break;
        };

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .unwrap_or_else(|_| panic!("Failed to connect to shard {index} ({key})"));

        shards.push(pool);
        index += 1;
    }

    assert!(
        !shards.is_empty(),
        "No shard database URLs found. Expected SHARD_0_DATABASE_URL, SHARD_1_DATABASE_URL, ..."
    );

    shards
}