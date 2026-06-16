use axum::Router;
use axum::routing::{delete, get, post};

use crate::state::AppState;

mod create;
mod delete;
mod list;
mod serve;

use create::create_gallery_image;
use delete::delete_gallery_image;
use list::list_gallery_images;
use serve::serve_gallery_image;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/projects/{slug}/gallery", get(list_gallery_images))
        .route("/projects/{slug}/gallery", post(create_gallery_image))
        .route("/projects/{slug}/gallery/{image}", get(serve_gallery_image))
        .route(
            "/projects/{slug}/gallery/{image}",
            delete(delete_gallery_image),
        )
}
