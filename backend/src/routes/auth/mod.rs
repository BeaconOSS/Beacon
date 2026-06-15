mod discord;
mod github;
mod login;
mod oauth;
mod register;
mod session;
mod turnstile;

use axum::Router;
use axum::routing::{get, post};

use crate::state::AppState;

use discord::{discord_callback, discord_start};
use github::{github_callback, github_start};
use login::login;
use register::register;
use session::{logout, me};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/me", get(me))
        .route("/auth/github", get(github_start))
        .route("/auth/github/callback", get(github_callback))
        .route("/auth/discord", get(discord_start))
        .route("/auth/discord/callback", get(discord_callback))
}
