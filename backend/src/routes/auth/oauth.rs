use argon2::password_hash::rand_core::{OsRng, RngCore};
use axum_extra::extract::cookie::{Cookie, SameSite};
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use serde::Deserialize;
use sqlx::{PgPool, Row};
use time::Duration;

use crate::session;

pub const STATE_COOKIE: &str = "beacon_oauth_state";

#[derive(Deserialize)]
pub struct CallbackQuery {
    pub code: Option<String>,
    pub state: Option<String>,
    pub error: Option<String>,
}

pub fn generate_state() -> String {
    let mut bytes = [0u8; 32];
    OsRng.fill_bytes(&mut bytes);
    URL_SAFE_NO_PAD.encode(bytes)
}

pub fn state_cookie(value: String) -> Cookie<'static> {
    let mut cookie = Cookie::new(STATE_COOKIE, value);
    configure(&mut cookie);
    cookie.set_max_age(Duration::minutes(10));
    cookie
}

pub fn clear_state_cookie() -> Cookie<'static> {
    let mut cookie = Cookie::new(STATE_COOKIE, "");
    configure(&mut cookie);
    cookie.set_max_age(Duration::ZERO);
    cookie
}

fn configure(cookie: &mut Cookie<'static>) {
    cookie.set_http_only(true);
    cookie.set_same_site(SameSite::Lax);
    cookie.set_path("/");
    cookie.set_secure(session::cookie_secure());
}

pub async fn upsert_user(
    pool: &PgPool,
    provider: &str,
    provider_user_id: &str,
    email: &str,
    username_hint: &str,
) -> Result<String, sqlx::Error> {
    if let Some(row) = sqlx::query(
        "select user_id::text as user_id from user_identities \
         where provider = $1 and provider_user_id = $2",
    )
    .bind(provider)
    .bind(provider_user_id)
    .fetch_optional(pool)
    .await?
    {
        return Ok(row.get("user_id"));
    }

    if let Some(row) = sqlx::query("select id::text as id from users where email = $1")
        .bind(email)
        .fetch_optional(pool)
        .await?
    {
        let user_id: String = row.get("id");
        sqlx::query(
            "insert into user_identities (user_id, provider, provider_user_id) \
             values ($1::uuid, $2, $3)",
        )
        .bind(&user_id)
        .bind(provider)
        .bind(provider_user_id)
        .execute(pool)
        .await?;
        return Ok(user_id);
    }

    let username = unique_username(pool, username_hint).await?;
    let row = sqlx::query(
        r#"
        with new_user as (
            insert into users (username, email)
            values ($1, $2)
            returning id
        ), new_identity as (
            insert into user_identities (user_id, provider, provider_user_id)
            select id, $3, $4 from new_user
        )
        select id::text as id from new_user
        "#,
    )
    .bind(&username)
    .bind(email)
    .bind(provider)
    .bind(provider_user_id)
    .fetch_one(pool)
    .await?;

    Ok(row.get("id"))
}

async fn unique_username(pool: &PgPool, login: &str) -> Result<String, sqlx::Error> {
    let base = if login.is_empty() { "user" } else { login };
    let mut candidate = base.to_string();
    let mut suffix = 1;

    loop {
        let taken = sqlx::query("select 1 from users where username = $1")
            .bind(&candidate)
            .fetch_optional(pool)
            .await?
            .is_some();

        if !taken {
            return Ok(candidate);
        }

        suffix += 1;
        candidate = format!("{base}{suffix}");
    }
}
