use axum::response::{IntoResponse, Response};
use axum::{extract::Path, extract::State, http::StatusCode, http::header};
use sqlx::Row;

use crate::error::AppError;
use crate::storage::Storage;

pub async fn serve_gallery_image(
    State(pool): State<sqlx::PgPool>,
    State(storage): State<Storage>,
    Path((slug, image_id)): Path<(String, String)>,
) -> Result<Response, AppError> {
    let row = sqlx::query(
        r#"
        select g.storage_key, g.content_type
        from gallery_images g
        join projects p on p.id = g.project_id
        where p.slug = $1 and p.published = true and g.id = $2::uuid
        "#,
    )
    .bind(&slug)
    .bind(&image_id)
    .fetch_optional(&pool)
    .await?;

    let Some(row) = row else {
        return Err(AppError::not_found("image not found"));
    };
    let storage_key: String = row.get("storage_key");
    let content_type: String = row.get("content_type");

    let bytes = storage
        .get(&storage_key)
        .await
        .map_err(|_| AppError::internal("could not read image"))?;

    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, content_type)],
        bytes,
    )
        .into_response())
}
