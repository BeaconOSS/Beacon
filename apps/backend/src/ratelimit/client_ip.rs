use std::net::{IpAddr, SocketAddr};

use axum::extract::ConnectInfo;
use axum::http::{Extensions, HeaderMap};

/// Header our own Nuxt frontend sets during server-side rendering to carry the
/// real browser IP. Caddy does not touch it, so it survives the SSR -> backend
/// hop. It is only trusted for read traffic (see [`client_ip_from`]).
const FORWARDED_IP_HEADER: &str = "x-beacon-forwarded-ip";

/// The IP Caddy records for the immediate TCP peer. Unspoofable by the client
/// because Caddy overwrites whatever the client sent.
const REAL_IP_HEADER: &str = "x-real-ip";

/// Resolve the real client IP for rate limiting, accounting for the reverse
/// proxy and server-side rendering.
///
/// `trust_forwarded` should be true only for low-stakes read traffic: those
/// requests may originate from the Nuxt SSR server (which appears as the VPS's
/// own address to Caddy), so we honour the browser IP it forwards in
/// [`FORWARDED_IP_HEADER`]. For credential/write/upload traffic this is left
/// false and only Caddy's unspoofable [`REAL_IP_HEADER`] (or, in local dev, the
/// socket address) is used.
pub fn client_ip_from(
    headers: &HeaderMap,
    extensions: &Extensions,
    trust_forwarded: bool,
) -> String {
    if trust_forwarded && let Some(ip) = parse_ip_header(headers, FORWARDED_IP_HEADER) {
        return ip;
    }

    if let Some(ip) = parse_ip_header(headers, REAL_IP_HEADER) {
        return ip;
    }

    if let Some(ConnectInfo(addr)) = extensions.get::<ConnectInfo<SocketAddr>>() {
        return addr.ip().to_string();
    }

    "unknown".to_string()
}

fn parse_ip_header(headers: &HeaderMap, name: &str) -> Option<String> {
    headers
        .get(name)
        .and_then(|value| value.to_str().ok())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .and_then(|value| value.parse::<IpAddr>().ok())
        .map(|ip| ip.to_string())
}
