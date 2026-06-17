//! In-process rate limiting.
//!
//! A per-class token-bucket limiter (see [`limiter::RateLimiter`]) applied as
//! axum middleware. Keys are derived per request from the session cookie (for
//! authenticated writes) or the client IP, with no database lookups on the hot
//! path. Suitable for a single backend instance behind a trusted reverse proxy;
//! move to a shared store (e.g. Redis) only when scaling to multiple instances.

mod bucket;
mod classify;
mod client_ip;
mod limiter;
mod middleware;
mod store;

use std::sync::Arc;

use axum::Router;
use axum::middleware::from_fn_with_state;

pub use limiter::RateLimiter;

use middleware::enforce;

/// Wrap a fully-built `Router` with the global rate-limit middleware.
///
/// Apply this after `.with_state(...)` (so the router is stateless) and after
/// the tracing/CORS layers, near the outside of the stack. The class for each
/// request is derived from its method + path inside the middleware, so no
/// routing code needs to change.
pub fn apply(router: Router, limiter: Arc<RateLimiter>) -> Router {
    router.layer(from_fn_with_state(limiter, enforce))
}
