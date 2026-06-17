use std::time::Instant;

/// A single token bucket: `capacity` tokens that refill at `refill_per_sec`.
///
/// Each accepted request removes one token. When empty, the caller is limited
/// until enough time has passed to refill a whole token.
#[derive(Clone, Copy)]
pub struct Bucket {
    tokens: f64,
    last_refill: Instant,
}

impl Bucket {
    pub fn new(capacity: f64) -> Self {
        Self {
            tokens: capacity,
            last_refill: Instant::now(),
        }
    }

    /// Refill based on elapsed time, then try to spend one token.
    ///
    /// Returns `Ok(())` when a token was available, or `Err(retry_after_secs)`
    /// with the whole number of seconds until the next token is available.
    pub fn try_acquire(
        &mut self,
        capacity: f64,
        refill_per_sec: f64,
        now: Instant,
    ) -> Result<(), u64> {
        let elapsed = now
            .saturating_duration_since(self.last_refill)
            .as_secs_f64();
        self.last_refill = now;
        self.tokens = (self.tokens + elapsed * refill_per_sec).min(capacity);

        if self.tokens >= 1.0 {
            self.tokens -= 1.0;
            Ok(())
        } else {
            let missing = 1.0 - self.tokens;
            let secs = (missing / refill_per_sec).ceil() as u64;
            Err(secs.max(1))
        }
    }

    /// Whether this bucket is full (used by the GC to evict idle entries).
    pub fn is_full(&self, capacity: f64, refill_per_sec: f64, now: Instant) -> bool {
        let elapsed = now
            .saturating_duration_since(self.last_refill)
            .as_secs_f64();
        (self.tokens + elapsed * refill_per_sec) >= capacity
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    #[test]
    fn spends_full_burst_then_limits() {
        let mut bucket = Bucket::new(3.0);
        let now = Instant::now();
        assert!(bucket.try_acquire(3.0, 1.0, now).is_ok());
        assert!(bucket.try_acquire(3.0, 1.0, now).is_ok());
        assert!(bucket.try_acquire(3.0, 1.0, now).is_ok());
        assert!(bucket.try_acquire(3.0, 1.0, now).is_err());
    }

    #[test]
    fn refills_over_time() {
        let mut bucket = Bucket::new(2.0);
        let start = Instant::now();
        assert!(bucket.try_acquire(2.0, 1.0, start).is_ok());
        assert!(bucket.try_acquire(2.0, 1.0, start).is_ok());
        assert!(bucket.try_acquire(2.0, 1.0, start).is_err());

        // One token refills after a full second at 1 token/sec.
        let later = start + Duration::from_secs(1);
        assert!(bucket.try_acquire(2.0, 1.0, later).is_ok());
        assert!(bucket.try_acquire(2.0, 1.0, later).is_err());
    }

    #[test]
    fn retry_after_is_at_least_one_second() {
        let mut bucket = Bucket::new(1.0);
        let now = Instant::now();
        assert!(bucket.try_acquire(1.0, 0.5, now).is_ok());
        match bucket.try_acquire(1.0, 0.5, now) {
            Err(secs) => assert!(secs >= 1),
            Ok(()) => panic!("expected to be limited"),
        }
    }

    #[test]
    fn full_bucket_is_evictable() {
        let bucket = Bucket::new(5.0);
        assert!(bucket.is_full(5.0, 1.0, Instant::now()));
    }
}
