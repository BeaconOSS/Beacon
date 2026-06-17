use axum::Router;
use axum::routing::{get, post};

use crate::state::AppState;

mod notes;
mod pending;
mod queue;
mod review;

use notes::{add_moderator_note, list_moderator_notes};
use pending::pending_review;
use queue::queue;
use review::review;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/moderation/projects", get(queue))
        .route("/projects/{slug}/review", post(review))
        .route("/projects/{slug}/pending", get(pending_review))
        .route(
            "/projects/{slug}/moderator-notes",
            get(list_moderator_notes).post(add_moderator_note),
        )
}
