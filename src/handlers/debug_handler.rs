use axum::{extract::State, Json};

use crate::{models::responses::shard_stats_response::ShardStat, AppState};

#[utoipa::path(
    get,
    path = "/debug/shards",
    responses(
        (status = 200, description = "Per-shard connection pool stats", body = [ShardStat])
    )
)]
pub async fn shard_stats(State(state): State<AppState>) -> Json<Vec<ShardStat>> {
    let stats = state
        .shards
        .iter()
        .enumerate()
        .map(|(shard_index, pool)| {
            let total_connections = pool.size();
            let idle_connections = pool.num_idle();

            ShardStat {
                shard_index,
                max_connections: pool.options().get_max_connections(),
                total_connections,
                idle_connections,
                in_use_connections: total_connections - idle_connections as u32,
            }
        })
        .collect();

    Json(stats)
}
