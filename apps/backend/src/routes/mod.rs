use axum::{
    Router,
    http::{Method, header::CONTENT_TYPE},
};
use sqlx::PgPool;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use crate::config::Config;
use crate::ratelimit::{self, RateLimiter};
use crate::state::AppState;
use crate::storage::Storage;

mod access;
mod auth;
mod categories;
mod gallery;
mod health;
mod moderation;
mod owner;
mod projects;
mod sql;
mod versions;

pub fn router(pool: PgPool, storage: Storage, config: &Config) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(
            config
                .frontend_url
                .parse::<axum::http::HeaderValue>()
                .expect("invalid FRONTEND_URL"),
        )
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_headers([CONTENT_TYPE])
        .allow_credentials(true);

    let app = Router::new()
        .merge(health::routes())
        .merge(categories::routes())
        .merge(projects::routes())
        .merge(versions::routes())
        .merge(gallery::routes())
        .merge(moderation::routes())
        .merge(auth::routes())
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .with_state(AppState::new(pool, storage, config));

    if config.rate_limit_enabled {
        ratelimit::apply(app, RateLimiter::start())
    } else {
        app
    }
}
