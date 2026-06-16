use axum::Router;
use axum::routing::{get, patch, post};

use crate::state::AppState;

mod create;
mod detail;
mod icon;
mod list;
mod settings;
mod submit;
mod update;

use create::create;
use detail::detail;
use icon::{delete_icon, serve_icon, upload_icon};
use list::list;
use settings::settings;
use submit::submit;
use update::update;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/projects", get(list))
        .route("/projects", post(create))
        .route("/projects/{slug}", get(detail))
        .route("/projects/{slug}", patch(update))
        .route("/projects/{slug}/settings", get(settings))
        .route("/projects/{slug}/submit", post(submit))
        .route(
            "/projects/{slug}/icon",
            post(upload_icon).delete(delete_icon).get(serve_icon),
        )
}
