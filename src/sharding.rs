/// Routes by an id that already exists (a post's author, a comment/vote's post).
/// id % shard_count always lands on the shard that generated the id, because
/// each shard's sequence is configured with INCREMENT BY shard_count.
pub fn shard_for_id(id: i64, shard_count: i64) -> usize {
    id.rem_euclid(shard_count) as usize
}

/// Routes a brand-new user, who has no id yet, by hashing their username.
/// Deterministic: the same username always lands on the same shard.
pub fn shard_for_username(username: &str, shard_count: i64) -> usize {
    let mut hash: u64 = 0xcbf29ce484222325; // FNV-1a offset basis
    for byte in username.as_bytes() {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(0x100000001b3); // FNV-1a prime
    }
    (hash % shard_count as u64) as usize
}
