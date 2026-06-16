use axum::Router;
use axum::routing::{delete, get, patch, post};

use crate::state::AppState;

mod analytics;
mod create;
mod delete;
mod detail;
mod icon;
mod interactions;
mod list;
mod members;
mod settings;
mod submit;
mod update;

use analytics::analytics;
use create::create;
use delete::delete_project;
use detail::detail;
use icon::{delete_icon, serve_icon, upload_icon};
use interactions::{toggle_heart, toggle_save};
use list::list;
use members::{add_member, list_members, remove_member};
use settings::settings;
use submit::submit;
use update::update;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/projects", get(list))
        .route("/projects", post(create))
        .route("/projects/{slug}", get(detail))
        .route("/projects/{slug}", patch(update))
        .route("/projects/{slug}", delete(delete_project))
        .route("/projects/{slug}/settings", get(settings))
        .route("/projects/{slug}/submit", post(submit))
        .route("/projects/{slug}/analytics", get(analytics))
        .route("/projects/{slug}/heart", post(toggle_heart))
        .route("/projects/{slug}/save", post(toggle_save))
        .route(
            "/projects/{slug}/members",
            get(list_members).post(add_member),
        )
        .route("/projects/{slug}/members/{user_id}", delete(remove_member))
        .route(
            "/projects/{slug}/icon",
            post(upload_icon).delete(delete_icon).get(serve_icon),
        )
}
