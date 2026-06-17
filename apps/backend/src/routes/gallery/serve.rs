use axum::response::{IntoResponse, Response};
use axum::{extract::Path, extract::State, http::StatusCode, http::header};
use axum_extra::extract::cookie::CookieJar;
use sqlx::Row;

use crate::error::AppError;
use crate::routes::access::project_for_viewer;
use crate::storage::Storage;

pub async fn serve_gallery_image(
    State(pool): State<sqlx::PgPool>,
    State(storage): State<Storage>,
    jar: CookieJar,
    Path((slug, image_id)): Path<(String, String)>,
) -> Result<Response, AppError> {
    let project_id = project_for_viewer(&pool, &jar, &slug).await?;

    let row = sqlx::query(
        r#"
        select g.storage_key, g.content_type
        from gallery_images g
        where g.project_id = $1::uuid and g.id = $2::uuid
        "#,
    )
    .bind(&project_id)
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
