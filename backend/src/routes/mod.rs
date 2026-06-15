use axum::{
    Router,
    http::{Method, header::CONTENT_TYPE},
    routing::{delete, get, post},
};
use sqlx::PgPool;
use tower_http::cors::CorsLayer;

use crate::state::AppState;
use crate::storage::Storage;

mod auth;
mod categories;
mod health;
mod projects;

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
        .route("/health", get(health::health))
        .route("/categories", get(categories::list))
        .route("/projects", get(projects::list))
        .route("/projects", post(projects::create))
        .route("/projects/{slug}", get(projects::detail))
        .route("/projects/{slug}/versions", get(projects::list_versions))
        .route("/projects/{slug}/versions", post(projects::create_version))
        .route(
            "/projects/{slug}/versions/{version}/download",
            get(projects::download_version),
        )
        .route("/projects/{slug}/gallery", get(projects::list_gallery_images))
        .route("/projects/{slug}/gallery", post(projects::create_gallery_image))
        .route(
            "/projects/{slug}/gallery/{image}",
            get(projects::serve_gallery_image),
        )
        .route(
            "/projects/{slug}/gallery/{image}",
            delete(projects::delete_gallery_image),
        )
        .route("/register", post(auth::register))
        .route("/login", post(auth::login))
        .route("/logout", post(auth::logout))
        .route("/me", get(auth::me))
        .route("/auth/github", get(auth::github_start))
        .route("/auth/github/callback", get(auth::github_callback))
        .route("/auth/discord", get(auth::discord_start))
        .route("/auth/discord/callback", get(auth::discord_callback))
        .layer(cors)
        .with_state(AppState::from_env(pool, storage, frontend_url))
}
