use axum::http::Method;

use super::limiter::RateClass;

/// Map a request to its [`RateClass`] from method + path alone, so the entire
/// policy lives in one place and routing code stays untouched.
///
/// Unauthenticated credential endpoints are the tightest; uploads (which cost
/// storage + analyzer work) are next; other authenticated writes are moderate;
/// everything else (reads, asset serving) gets a generous ceiling.
pub fn classify(method: &Method, path: &str) -> RateClass {
    if is_auth_strict(method, path) {
        return RateClass::AuthStrict;
    }

    // Reads are never write-classified regardless of path.
    if method == Method::GET || method == Method::HEAD || method == Method::OPTIONS {
        return RateClass::Read;
    }

    if is_upload(method, path) {
        return RateClass::Upload;
    }

    // Any remaining mutating method (POST/PATCH/DELETE/PUT).
    RateClass::Mutate
}

fn is_auth_strict(method: &Method, path: &str) -> bool {
    if method == Method::POST && (path == "/login" || path == "/register") {
        return true;
    }
    // OAuth start + callback (GET) involve provider/token exchange and account
    // creation, so they are rate limited as credential traffic.
    path.starts_with("/auth/")
}

fn is_upload(method: &Method, path: &str) -> bool {
    if method != Method::POST {
        return false;
    }
    // Version uploads, icon uploads, and gallery image uploads.
    path.ends_with("/icon")
        || path.ends_with("/gallery")
        || (path.starts_with("/projects/") && path.ends_with("/versions"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn credentials_are_auth_strict() {
        assert_eq!(classify(&Method::POST, "/login"), RateClass::AuthStrict);
        assert_eq!(classify(&Method::POST, "/register"), RateClass::AuthStrict);
        assert_eq!(
            classify(&Method::GET, "/auth/discord/callback"),
            RateClass::AuthStrict
        );
    }

    #[test]
    fn reads_are_read_class() {
        assert_eq!(classify(&Method::GET, "/projects"), RateClass::Read);
        assert_eq!(classify(&Method::HEAD, "/projects/42"), RateClass::Read);
        assert_eq!(
            classify(&Method::GET, "/projects/42/versions"),
            RateClass::Read
        );
    }

    #[test]
    fn uploads_are_upload_class() {
        assert_eq!(
            classify(&Method::POST, "/projects/42/icon"),
            RateClass::Upload
        );
        assert_eq!(
            classify(&Method::POST, "/projects/42/gallery"),
            RateClass::Upload
        );
        assert_eq!(
            classify(&Method::POST, "/projects/42/versions"),
            RateClass::Upload
        );
    }

    #[test]
    fn other_writes_are_mutate_class() {
        assert_eq!(classify(&Method::POST, "/projects"), RateClass::Mutate);
        assert_eq!(classify(&Method::PATCH, "/projects/42"), RateClass::Mutate);
        assert_eq!(classify(&Method::DELETE, "/projects/42"), RateClass::Mutate);
    }
}
