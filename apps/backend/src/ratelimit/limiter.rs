use std::sync::Arc;
use std::time::Duration;

use super::store::LimitStore;

/// Which traffic class a route belongs to. Each class has independent limits and
/// its own keying strategy (see [`RateLimiter::check`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RateClass {
    /// Unauthenticated credential endpoints (login, register, OAuth callbacks).
    /// Always keyed by client IP and kept deliberately tight.
    AuthStrict,
    /// File uploads (versions, icons, gallery) - storage + analyzer cost.
    Upload,
    /// Authenticated writes (create, heart, save, members, reviews, ...).
    Mutate,
    /// Public reads (browse, detail, asset serving).
    Read,
}

/// Holds one [`LimitStore`] per [`RateClass`] and runs a background eviction task.
pub struct RateLimiter {
    auth_strict: LimitStore,
    upload: LimitStore,
    mutate: LimitStore,
    read: LimitStore,
}

impl RateLimiter {
    /// Build the limiter with the production tiers and start the GC loop.
    pub fn start() -> Arc<Self> {
        let limiter = Arc::new(Self {
            auth_strict: LimitStore::new(10, 60),
            upload: LimitStore::new(20, 300),
            mutate: LimitStore::new(60, 60),
            read: LimitStore::new(300, 60),
        });

        let gc = Arc::clone(&limiter);
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(60));
            interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
            loop {
                interval.tick().await;
                gc.evict_idle();
            }
        });

        limiter
    }

    fn store(&self, class: RateClass) -> &LimitStore {
        match class {
            RateClass::AuthStrict => &self.auth_strict,
            RateClass::Upload => &self.upload,
            RateClass::Mutate => &self.mutate,
            RateClass::Read => &self.read,
        }
    }

    /// Try to admit one request in `class` for `identity` (a user id or client
    /// IP). Returns `Err(retry_after_secs)` when the caller is limited.
    pub fn check(&self, class: RateClass, identity: &str) -> Result<(), u64> {
        let prefix = match class {
            RateClass::AuthStrict => "auth",
            RateClass::Upload => "upload",
            RateClass::Mutate => "mutate",
            RateClass::Read => "read",
        };
        let key = format!("{prefix}:{identity}");
        self.store(class).check(&key)
    }

    fn evict_idle(&self) {
        self.auth_strict.evict_idle();
        self.upload.evict_idle();
        self.mutate.evict_idle();
        self.read.evict_idle();
    }
}
