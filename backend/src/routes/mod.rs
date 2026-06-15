use axum::{
    Router,
    http::{Method, header::CONTENT_TYPE},
    routing::{get, post},
};
use sqlx::PgPool;
use tower_http::cors::CorsLayer;

use crate::state::AppState;

mod auth;
mod health;

pub fn router(pool: PgPool, frontend_url: &str) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(
            frontend_url
                .parse::<axum::http::HeaderValue>()
                .expect("invalid FRONTEND_URL"),
        )
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([CONTENT_TYPE])
        .allow_credentials(true);

    Router::new()
        .route("/health", get(health::health))
        .route("/register", post(auth::register))
        .route("/login", post(auth::login))
        .route("/logout", post(auth::logout))
        .route("/me", get(auth::me))
        .route("/auth/github", get(auth::github_start))
        .route("/auth/github/callback", get(auth::github_callback))
        .layer(cors)
        .with_state(AppState::from_env(pool, frontend_url))
}
