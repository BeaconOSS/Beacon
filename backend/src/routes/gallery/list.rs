use axum::response::{IntoResponse, Response};
use axum::{Json, extract::Path, extract::State, http::StatusCode};
use serde::Serialize;
use serde_json::json;
use sqlx::Row;

use crate::error::AppError;

#[derive(Serialize)]
struct GalleryImage {
    id: String,
    caption: String,
    url: String,
}

pub async fn list_gallery_images(
    State(pool): State<sqlx::PgPool>,
    Path(slug): Path<String>,
) -> Result<Response, AppError> {
    let project =
        sqlx::query("select id::text as id from projects where slug = $1 and published = true")
            .bind(&slug)
            .fetch_optional(&pool)
            .await?;

    let Some(project) = project else {
        return Err(AppError::not_found("project not found"));
    };
    let project_id: String = project.get("id");

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
