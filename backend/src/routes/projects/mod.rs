use axum::Router;
use axum::routing::{get, post};

use crate::state::AppState;

mod create;
mod detail;
mod list;

use create::create;
use detail::detail;
use list::list;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/projects", get(list))
        .route("/projects", post(create))
        .route("/projects/{slug}", get(detail))
}
