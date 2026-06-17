use std::sync::Arc;

use axum::Json;
use axum::extract::{Request, State};
use axum::http::{StatusCode, header::RETRY_AFTER};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum_extra::extract::cookie::CookieJar;
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use serde_json::json;
use sha2::{Digest, Sha256};

use super::classify::classify;
use super::client_ip::client_ip_from;
use super::limiter::{RateClass, RateLimiter};
use crate::session::SESSION_COOKIE;

/// Resolve the limit identity for a request.
///
/// Authenticated write/upload traffic is keyed per session (so one signed-in
/// user shares a budget across devices/IPs and cannot multiply it by rotating
/// IPs), using a hash of the opaque session token - no database lookup. All
/// other traffic, and any request without a session cookie, is keyed by client
/// IP.
fn identity_for(class: RateClass, jar: &CookieJar, client_ip: &str) -> String {
    let session_keyed = matches!(class, RateClass::Mutate | RateClass::Upload);
    if session_keyed && let Some(token) = jar.get(SESSION_COOKIE).map(|cookie| cookie.value()) {
        let digest = Sha256::digest(token.as_bytes());
        return format!("s:{}", URL_SAFE_NO_PAD.encode(digest));
    }
    format!("ip:{client_ip}")
}

/// Axum middleware enforcing the rate-limit policy for every request.
///
/// The class is derived from method + path (see [`classify`]); on limit it
/// short-circuits with `429 Too Many Requests`, a `Retry-After` header, and the
/// standard `{ "error": ... }` body shape.
pub async fn enforce(
    State(limiter): State<Arc<RateLimiter>>,
    jar: CookieJar,
    request: Request,
    next: Next,
) -> Response {
    let class = classify(request.method(), request.uri().path());
    let trust_forwarded = matches!(class, RateClass::Read);
    let client_ip = client_ip_from(request.headers(), request.extensions(), trust_forwarded);
    let identity = identity_for(class, &jar, &client_ip);

    match limiter.check(class, &identity) {
        Ok(()) => next.run(request).await,
        Err(retry_after) => (
            StatusCode::TOO_MANY_REQUESTS,
            [(RETRY_AFTER, retry_after.to_string())],
            Json(json!({ "error": "too many requests, please slow down" })),
        )
            .into_response(),
    }
}
