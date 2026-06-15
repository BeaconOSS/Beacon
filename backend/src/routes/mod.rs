use axum::{
    Router,
    http::{Method, header::CONTENT_TYPE},
};
use sqlx::PgPool;
use tower_http::cors::CorsLayer;

use crate::state::AppState;
use crate::storage::Storage;

mod auth;
mod categories;
mod gallery;
mod health;
mod projects;
mod versions;

pub fn router(pool: PgPool, storage: Storage, frontend_url: &str) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(
            frontend_url
                .parse::<axum::http::HeaderValue>()
                .expect("invalid FRONTEND_URL"),
        )
        .allow_methods([Method::GET, Method::POST, Method::DELETE])
        .allow_headers([CONTENT_TYPE])
        .allow_credentials(true);

    Router::new()
        .merge(health::routes())
        .merge(categories::routes())
        .merge(projects::routes())
        .merge(versions::routes())
        .merge(gallery::routes())
        .merge(auth::routes())
        .layer(cors)
        .with_state(AppState::from_env(pool, storage, frontend_url))
}
