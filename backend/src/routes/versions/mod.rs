use axum::Router;
use axum::routing::{delete, get, post};

use crate::state::AppState;

mod create;
mod delete;
mod download;
mod list;

use create::create_version;
use delete::delete_version;
use download::download_version;
use list::list_versions;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/projects/{slug}/versions", get(list_versions))
        .route("/projects/{slug}/versions", post(create_version))
        .route(
            "/projects/{slug}/versions/{version}/download",
            get(download_version),
        )
        .route(
            "/projects/{slug}/versions/{version}",
            delete(delete_version),
        )
}
