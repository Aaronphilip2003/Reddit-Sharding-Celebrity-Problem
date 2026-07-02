use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct ShardStat {
    pub shard_index: usize,
    pub max_connections: u32,
    pub total_connections: u32,
    pub idle_connections: usize,
    pub in_use_connections: u32,
}
