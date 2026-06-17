use axum::response::{IntoResponse, Response};
use axum::{Json, extract::Path, extract::State, http::StatusCode};
use axum_extra::extract::cookie::CookieJar;
use serde::Serialize;
use serde_json::json;
use sqlx::Row;

use crate::error::AppError;
use crate::routes::access::project_for_viewer;

#[derive(Serialize)]
struct GalleryImage {
    id: String,
    caption: String,
    url: String,
}

pub async fn list_gallery_images(
    State(pool): State<sqlx::PgPool>,
    jar: CookieJar,
    Path(slug): Path<String>,
) -> Result<Response, AppError> {
    let project_id = project_for_viewer(&pool, &jar, &slug).await?;

    let rows = sqlx::query(
        r#"
        select id::text as id, caption
        from gallery_images
        where project_id = $1::uuid
        order by position, created_at
        "#,
    )
    .bind(&project_id)
    .fetch_all(&pool)
    .await?;

    let images: Vec<GalleryImage> = rows
        .into_iter()
        .map(|row| {
            let id: String = row.get("id");
            GalleryImage {
                url: format!("/projects/{slug}/gallery/{id}"),
                caption: row.get("caption"),
                id,
            }
        })
        .collect();
    Ok((StatusCode::OK, Json(json!({ "images": images }))).into_response())
}
