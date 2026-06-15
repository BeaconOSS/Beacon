use argon2::password_hash::rand_core::{OsRng, RngCore};
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect, Response};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use serde::Deserialize;
use sqlx::{PgPool, Row};
use time::Duration;

use crate::error::error;
use crate::session;
use crate::state::{AppState, GithubOauth};

pub const STATE_COOKIE: &str = "beacon_oauth_state";
const AUTHORIZE_URL: &str = "https://github.com/login/oauth/authorize";
const TOKEN_URL: &str = "https://github.com/login/oauth/access_token";
const USER_URL: &str = "https://api.github.com/user";
const EMAILS_URL: &str = "https://api.github.com/user/emails";
const USER_AGENT: &str = "beacon-backend";
const SCOPE: &str = "read:user user:email";


pub async fn github_start(State(state): State<AppState>, jar: CookieJar) -> Response {
    let Some(github) = state.github.as_ref() else {
        return error(StatusCode::NOT_FOUND, "github sign-in is not configured").into_response();
    };

    let oauth_state = generate_state();
    let redirect_uri = format!("{}/auth/github/callback", state.redirect_base);

    let authorize_url = format!(
        "{AUTHORIZE_URL}?client_id={}&redirect_uri={}&scope={}&state={}&allow_signup=true",
        urlencoding::encode(&github.client_id),
        urlencoding::encode(&redirect_uri),
        urlencoding::encode(SCOPE),
        urlencoding::encode(&oauth_state),
    );

    let jar = jar.add(state_cookie(oauth_state));
    (jar, Redirect::to(&authorize_url)).into_response()
}

fn generate_state() -> String {
    let mut bytes = [0u8; 32];
    OsRng.fill_bytes(&mut bytes);
    URL_SAFE_NO_PAD.encode(bytes)
}

fn state_cookie(value: String) -> Cookie<'static> {
    let mut cookie = Cookie::new(STATE_COOKIE, value);
    cookie.set_http_only(true);
    cookie.set_same_site(SameSite::Lax);
    cookie.set_path("/");
    cookie.set_secure(session::cookie_secure());
    cookie.set_max_age(Duration::minutes(10));
    cookie
}

#[derive(Deserialize)]
pub struct CallbackQuery {
    code: Option<String>,
    state: Option<String>,
    error: Option<String>,
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: Option<String>,
}

#[derive(Deserialize)]
struct GithubUser {
    id: i64,
    login: String,
    email: Option<String>,
}

#[derive(Deserialize)]
struct GithubEmail {
    email: String,
    primary: bool,
    verified: bool,
}

pub async fn github_callback(
    State(state): State<AppState>,
    jar: CookieJar,
    Query(query): Query<CallbackQuery>,
) -> Response {
    let login_url = format!("{}/login", state.frontend_url);
    let fail = |jar: CookieJar, reason: &str| {
        let jar = jar.add(clear_state_cookie());
        (
            jar,
            Redirect::to(&format!("{login_url}?error={reason}")),
        )
            .into_response()
    };

    let Some(github) = state.github.as_ref() else {
        return error(StatusCode::NOT_FOUND, "github sign-in is not configured").into_response();
    };

    if query.error.is_some() {
        return fail(jar, "github_denied");
    }

    let (Some(code), Some(returned_state)) = (query.code, query.state) else {
        return fail(jar, "github_invalid");
    };

    let expected_state = jar.get(STATE_COOKIE).map(|c| c.value().to_string());
    if expected_state.as_deref() != Some(returned_state.as_str()) {
        return fail(jar, "github_state");
    }

    let redirect_uri = format!("{}/auth/github/callback", state.redirect_base);
    let token = match exchange_code(github, &code, &redirect_uri).await {
        Some(token) => token,
        None => return fail(jar, "github_token"),
    };

    let user = match fetch_user(&token).await {
        Some(user) => user,
        None => return fail(jar, "github_user"),
    };

    let email = match user.email.clone() {
        Some(email) => email,
        None => match fetch_primary_email(&token).await {
            Some(email) => email,
            None => return fail(jar, "github_email"),
        },
    };

    let user_id = match upsert_user(&state.pool, &user, &email).await {
        Ok(id) => id,
        Err(_) => return fail(jar, "github_account"),
    };

    let session_token = match session::create(&state.pool, &user_id).await {
        Ok(token) => token,
        Err(_) => return fail(jar, "github_session"),
    };

    let jar = jar
        .add(clear_state_cookie())
        .add(session::build_cookie(session_token));
    (jar, Redirect::to(&state.frontend_url)).into_response()
}

async fn exchange_code(github: &GithubOauth, code: &str, redirect_uri: &str) -> Option<String> {
    let response = reqwest::Client::new()
        .post(TOKEN_URL)
        .header(reqwest::header::ACCEPT, "application/json")
        .header(reqwest::header::USER_AGENT, USER_AGENT)
        .form(&[
            ("client_id", github.client_id.as_str()),
            ("client_secret", github.client_secret.as_str()),
            ("code", code),
            ("redirect_uri", redirect_uri),
        ])
        .send()
        .await
        .ok()?;

    response.json::<TokenResponse>().await.ok()?.access_token
}

async fn fetch_user(token: &str) -> Option<GithubUser> {
    reqwest::Client::new()
        .get(USER_URL)
        .header(reqwest::header::ACCEPT, "application/vnd.github+json")
        .header(reqwest::header::USER_AGENT, USER_AGENT)
        .bearer_auth(token)
        .send()
        .await
        .ok()?
        .json::<GithubUser>()
        .await
        .ok()
}

async fn fetch_primary_email(token: &str) -> Option<String> {
    let emails = reqwest::Client::new()
        .get(EMAILS_URL)
        .header(reqwest::header::ACCEPT, "application/vnd.github+json")
        .header(reqwest::header::USER_AGENT, USER_AGENT)
        .bearer_auth(token)
        .send()
        .await
        .ok()?
        .json::<Vec<GithubEmail>>()
        .await
        .ok()?;

    emails
        .iter()
        .find(|e| e.primary && e.verified)
        .or_else(|| emails.iter().find(|e| e.verified))
        .map(|e| e.email.clone())
}

async fn upsert_user(pool: &PgPool, user: &GithubUser, email: &str) -> Result<String, sqlx::Error> {
    let provider_user_id = user.id.to_string();

    if let Some(row) = sqlx::query(
        "select user_id::text as user_id from user_identities \
         where provider = 'github' and provider_user_id = $1",
    )
    .bind(&provider_user_id)
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
             values ($1::uuid, 'github', $2)",
        )
        .bind(&user_id)
        .bind(&provider_user_id)
        .execute(pool)
        .await?;
        return Ok(user_id);
    }

    let username = unique_username(pool, &user.login).await?;
    let row = sqlx::query(
        r#"
        with new_user as (
            insert into users (username, email)
            values ($1, $2)
            returning id
        ), new_identity as (
            insert into user_identities (user_id, provider, provider_user_id)
            select id, 'github', $3 from new_user
        )
        select id::text as id from new_user
        "#,
    )
    .bind(&username)
    .bind(email)
    .bind(&provider_user_id)
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

fn clear_state_cookie() -> Cookie<'static> {
    let mut cookie = Cookie::new(STATE_COOKIE, "");
    cookie.set_http_only(true);
    cookie.set_same_site(SameSite::Lax);
    cookie.set_path("/");
    cookie.set_secure(session::cookie_secure());
    cookie.set_max_age(Duration::ZERO);
    cookie
}
