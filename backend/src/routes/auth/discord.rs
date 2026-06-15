use axum::extract::{Query, State};
use axum::response::{IntoResponse, Redirect, Response};
use axum_extra::extract::cookie::CookieJar;
use serde::Deserialize;

use crate::error::AppError;
use crate::routes::auth::oauth;
use crate::session;
use crate::state::{AppState, DiscordOauth};

const AUTHORIZE_URL: &str = "https://discord.com/oauth2/authorize";
const TOKEN_URL: &str = "https://discord.com/api/oauth2/token";
const USER_URL: &str = "https://discord.com/api/users/@me";
const SCOPE: &str = "identify email";

pub async fn discord_start(State(state): State<AppState>, jar: CookieJar) -> Response {
    let Some(discord) = state.discord.as_ref() else {
        return AppError::not_found("discord sign-in is not configured").into_response();
    };

    let oauth_state = oauth::generate_state();
    let redirect_uri = format!("{}/auth/discord/callback", state.redirect_base);

    let authorize_url = format!(
        "{AUTHORIZE_URL}?client_id={}&redirect_uri={}&response_type=code&scope={}&state={}",
        urlencoding::encode(&discord.client_id),
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
struct DiscordUser {
    id: String,
    username: String,
    email: Option<String>,
    #[serde(default)]
    verified: bool,
}

pub async fn discord_callback(
    State(state): State<AppState>,
    jar: CookieJar,
    Query(query): Query<oauth::CallbackQuery>,
) -> Response {
    let login_url = format!("{}/login", state.frontend_url);
    let fail = |jar: CookieJar, reason: &str| {
        let jar = jar.add(oauth::clear_state_cookie());
        (jar, Redirect::to(&format!("{login_url}?error={reason}"))).into_response()
    };

    let Some(discord) = state.discord.as_ref() else {
        return AppError::not_found("discord sign-in is not configured").into_response();
    };

    if query.error.is_some() {
        return fail(jar, "discord_denied");
    }

    let (Some(code), Some(returned_state)) = (query.code, query.state) else {
        return fail(jar, "discord_invalid");
    };

    let expected_state = jar.get(oauth::STATE_COOKIE).map(|c| c.value().to_string());
    if expected_state.as_deref() != Some(returned_state.as_str()) {
        return fail(jar, "discord_state");
    }

    let redirect_uri = format!("{}/auth/discord/callback", state.redirect_base);
    let token = match exchange_code(discord, &code, &redirect_uri).await {
        Some(token) => token,
        None => return fail(jar, "discord_token"),
    };

    let user = match fetch_user(&token).await {
        Some(user) => user,
        None => return fail(jar, "discord_user"),
    };

    let email = match user.email.as_deref() {
        Some(email) if user.verified => email.to_string(),
        _ => return fail(jar, "discord_email"),
    };

    let user_id =
        match oauth::upsert_user(&state.pool, "discord", &user.id, &email, &user.username).await {
            Ok(id) => id,
            Err(_) => return fail(jar, "discord_account"),
        };

    let session_token = match session::create(&state.pool, &user_id).await {
        Ok(token) => token,
        Err(_) => return fail(jar, "discord_session"),
    };

    let jar = jar
        .add(oauth::clear_state_cookie())
        .add(session::build_cookie(session_token));
    (jar, Redirect::to(&state.frontend_url)).into_response()
}

async fn exchange_code(discord: &DiscordOauth, code: &str, redirect_uri: &str) -> Option<String> {
    let response = reqwest::Client::new()
        .post(TOKEN_URL)
        .form(&[
            ("client_id", discord.client_id.as_str()),
            ("client_secret", discord.client_secret.as_str()),
            ("grant_type", "authorization_code"),
            ("code", code),
            ("redirect_uri", redirect_uri),
        ])
        .send()
        .await
        .ok()?;

    response.json::<TokenResponse>().await.ok()?.access_token
}

async fn fetch_user(token: &str) -> Option<DiscordUser> {
    reqwest::Client::new()
        .get(USER_URL)
        .bearer_auth(token)
        .send()
        .await
        .ok()?
        .json::<DiscordUser>()
        .await
        .ok()
}
