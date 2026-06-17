use axum::extract::{Query, State};
use axum::response::{IntoResponse, Redirect, Response};
use axum_extra::extract::cookie::CookieJar;
use serde::Deserialize;

use crate::error::AppError;
use crate::routes::auth::oauth;
use crate::session;
use crate::state::{AppState, GithubOauth};

const AUTHORIZE_URL: &str = "https://github.com/login/oauth/authorize";
const TOKEN_URL: &str = "https://github.com/login/oauth/access_token";
const USER_URL: &str = "https://api.github.com/user";
const EMAILS_URL: &str = "https://api.github.com/user/emails";
const USER_AGENT: &str = "beacon-backend";
const SCOPE: &str = "read:user user:email";

pub async fn github_start(State(state): State<AppState>, jar: CookieJar) -> Response {
    let Some(github) = state.github.as_ref() else {
        return AppError::not_found("github sign-in is not configured").into_response();
    };

    let oauth_state = oauth::generate_state();
    let redirect_uri = format!("{}/auth/github/callback", state.redirect_base);

    let authorize_url = format!(
        "{AUTHORIZE_URL}?client_id={}&redirect_uri={}&scope={}&state={}&allow_signup=true",
        urlencoding::encode(&github.client_id),
        urlencoding::encode(&redirect_uri),
        urlencoding::encode(SCOPE),
        urlencoding::encode(&oauth_state),
    );

    let jar = jar.add(oauth::state_cookie(oauth_state));
    (jar, Redirect::to(&authorize_url)).into_response()
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
    Query(query): Query<oauth::CallbackQuery>,
) -> Response {
    let login_url = format!("{}/login", state.frontend_url);
    let fail = |jar: CookieJar, reason: &str| {
        let jar = jar.add(oauth::clear_state_cookie());
        (jar, Redirect::to(&format!("{login_url}?error={reason}"))).into_response()
    };

    let Some(github) = state.github.as_ref() else {
        return AppError::not_found("github sign-in is not configured").into_response();
    };

    if query.error.is_some() {
        return fail(jar, "github_denied");
    }

    let (Some(code), Some(returned_state)) = (query.code, query.state) else {
        return fail(jar, "github_invalid");
    };

    let expected_state = jar.get(oauth::STATE_COOKIE).map(|c| c.value().to_string());
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

    let user_id = match oauth::upsert_user(
        &state.pool,
        "github",
        &user.id.to_string(),
        &email,
        &user.login,
    )
    .await
    {
        Ok(id) => id,
        Err(oauth::UpsertError::RegistrationClosed) => return fail(jar, "registration_closed"),
        Err(oauth::UpsertError::Sqlx(err)) => {
            tracing::error!(?err, "github oauth account upsert failed");
            return fail(jar, "github_account");
        }
    };

    let session_token = match session::create(&state.pool, &user_id).await {
        Ok(token) => token,
        Err(_) => return fail(jar, "github_session"),
    };

    let jar = jar
        .add(oauth::clear_state_cookie())
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
