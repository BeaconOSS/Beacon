use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::Mutex;
use std::time::Instant;

use super::bucket::Bucket;

const SHARD_COUNT: usize = 32;

/// One named limit class (e.g. auth, upload) with its own bucket parameters and
/// a sharded map of per-key buckets.
///
/// Keys are `class:identifier` strings (identifier = user id or client IP).
/// Sharding spreads lock contention so a 5k-concurrent-user burst does not
/// serialise on a single mutex.
pub struct LimitStore {
    capacity: f64,
    refill_per_sec: f64,
    shards: Vec<Mutex<HashMap<String, Bucket>>>,
}

impl LimitStore {
    pub fn new(capacity: u32, per_secs: u32) -> Self {
        let capacity = capacity.max(1) as f64;
        let refill_per_sec = capacity / per_secs.max(1) as f64;
        let mut shards = Vec::with_capacity(SHARD_COUNT);
        for _ in 0..SHARD_COUNT {
            shards.push(Mutex::new(HashMap::new()));
        }
        Self {
            capacity,
            refill_per_sec,
            shards,
        }
    }

    fn shard_for(&self, key: &str) -> &Mutex<HashMap<String, Bucket>> {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        key.hash(&mut hasher);
        let index = (hasher.finish() as usize) % self.shards.len();
        &self.shards[index]
    }

    /// Try to spend one token for `key`. `Ok(())` if allowed, `Err(retry_secs)`
    /// when the key is currently rate limited.
    pub fn check(&self, key: &str) -> Result<(), u64> {
        let now = Instant::now();
        let mut guard = self
            .shard_for(key)
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        let bucket = guard
            .entry(key.to_string())
            .or_insert_with(|| Bucket::new(self.capacity));
        bucket.try_acquire(self.capacity, self.refill_per_sec, now)
    }

    /// Drop full (idle) buckets to keep memory bounded between traffic bursts.
    pub fn evict_idle(&self) {
        let now = Instant::now();
        for shard in &self.shards {
            let mut guard = shard.lock().unwrap_or_else(|e| e.into_inner());
            guard.retain(|_, bucket| !bucket.is_full(self.capacity, self.refill_per_sec, now));
        }
    }
}
