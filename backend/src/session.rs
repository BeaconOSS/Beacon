use argon2::password_hash::rand_core::{OsRng, RngCore};
use axum_extra::extract::cookie::{Cookie, SameSite};
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use sha2::{Digest, Sha256};
use sqlx::{PgPool, Row};
use std::sync::OnceLock;
use time::Duration;

pub const SESSION_COOKIE: &str = "beacon_session";
const SESSION_DAYS: i32 = 30;

pub struct SessionUser {
    pub id: String,
    pub username: String,
    pub email: String,
}

pub async fn create(pool: &PgPool, user_id: &str) -> Result<String, sqlx::Error> {
    let token = generate_token();
    let token_hash = hash_token(&token);

    sqlx::query(
        r#"
        insert into sessions (user_id, token_hash, expires_at)
        values ($1::uuid, $2, now() + make_interval(days => $3))
        "#,
    )
    .bind(user_id)
    .bind(&token_hash)
    .bind(SESSION_DAYS)
    .execute(pool)
    .await?;

    Ok(token)
}

pub async fn lookup(pool: &PgPool, token: &str) -> Result<Option<SessionUser>, sqlx::Error> {
    let token_hash = hash_token(token);

    let row = sqlx::query(
        r#"
        select u.id::text as id, u.username, u.email
        from sessions s
        join users u on u.id = s.user_id
        where s.token_hash = $1 and s.expires_at > now()
        "#,
    )
    .bind(token_hash)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| SessionUser {
        id: r.get("id"),
        username: r.get("username"),
        email: r.get("email"),
    }))
}

pub async fn delete(pool: &PgPool, token: &str) -> Result<(), sqlx::Error> {
    let token_hash = hash_token(token);
    sqlx::query("delete from sessions where token_hash = $1")
        .bind(token_hash)
        .execute(pool)
        .await?;
    Ok(())
}

pub fn build_cookie(token: String) -> Cookie<'static> {
    let mut cookie = Cookie::new(SESSION_COOKIE, token);
    configure(&mut cookie);
    cookie.set_max_age(Duration::days(SESSION_DAYS as i64));
    cookie
}

pub fn clear_cookie() -> Cookie<'static> {
    let mut cookie = Cookie::new(SESSION_COOKIE, "");
    configure(&mut cookie);
    cookie.set_max_age(Duration::ZERO);
    cookie
}

fn generate_token() -> String {
    let mut bytes = [0u8; 32];
    let mut rng = OsRng;
    rng.fill_bytes(&mut bytes);
    URL_SAFE_NO_PAD.encode(bytes)
}

fn hash_token(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    URL_SAFE_NO_PAD.encode(hasher.finalize())
}

fn configure(cookie: &mut Cookie<'static>) {
    cookie.set_http_only(true);
    cookie.set_same_site(SameSite::Lax);
    cookie.set_path("/");
    cookie.set_secure(cookie_secure());
    if let Some(domain) = cookie_domain() {
        cookie.set_domain(domain);
    }
}

pub fn cookie_secure() -> bool {
    static SECURE: OnceLock<bool> = OnceLock::new();
    *SECURE.get_or_init(|| {
        std::env::var("COOKIE_SECURE")
            .map(|v| v == "true" || v == "1")
            .unwrap_or(false)
    })
}

pub fn cookie_domain() -> Option<String> {
    static DOMAIN: OnceLock<Option<String>> = OnceLock::new();
    DOMAIN
        .get_or_init(|| {
            std::env::var("COOKIE_DOMAIN")
                .ok()
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
        })
        .clone()
}

pub fn allow_registration() -> bool {
    static ALLOW: OnceLock<bool> = OnceLock::new();
    *ALLOW.get_or_init(|| {
        std::env::var("ALLOW_REGISTRATION")
            .map(|v| !(v == "false" || v == "0"))
            .unwrap_or(true)
    })
}
