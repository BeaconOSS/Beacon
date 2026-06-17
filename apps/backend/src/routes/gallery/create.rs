use axum::response::{IntoResponse, Response};
use axum::{Json, extract::Multipart, extract::Path, extract::State, http::StatusCode};
use serde_json::json;
use sqlx::Row;

use crate::error::AppError;
use crate::extract::AuthUser;
use crate::routes::owner::{ensure_not_in_review, require_project_owner};
use crate::storage::Storage;
use crate::utils::UploadForm;

const ALLOWED_IMAGE_TYPES: [(&str, &str); 4] = [
    ("image/png", "png"),
    ("image/jpeg", "jpg"),
    ("image/webp", "webp"),
    ("image/gif", "gif"),
];

pub async fn create_gallery_image(
    State(pool): State<sqlx::PgPool>,
    State(storage): State<Storage>,
    AuthUser(user): AuthUser,
    Path(slug): Path<String>,
    multipart: Multipart,
) -> Result<Response, AppError> {
    let project_id = require_project_owner(&pool, &slug, &user.id).await?;
    ensure_not_in_review(&pool, &project_id).await?;

    let mut form = UploadForm::collect(multipart).await?;

    let caption = form.text("caption");
    let image = form.take("image");
    let content_type = image
        .as_ref()
        .and_then(|field| field.content_type.clone())
        .unwrap_or_default();

    let Some(extension) = ALLOWED_IMAGE_TYPES
        .iter()
        .find(|(mime, _)| *mime == content_type)
        .map(|(_, ext)| *ext)
    else {
        return Err(AppError::bad_request("an image file is required"));
    };

    let Some(image) = image else {
        return Err(AppError::bad_request("an image file is required"));
    };
    let bytes = image.bytes;
    if bytes.is_empty() {
        return Err(AppError::bad_request("an image file is required"));
    }

    let image_id: String = sqlx::query("select gen_random_uuid()::text as id")
        .fetch_one(&pool)
        .await?
        .get("id");
    let storage_key = format!("{project_id}/gallery/{image_id}.{extension}");

    storage
        .put(&storage_key, &bytes, &content_type)
        .await
        .map_err(|_| AppError::internal("could not store image"))?;

    let row = sqlx::query(
        r#"
        insert into gallery_images (id, project_id, storage_key, caption, content_type, position)
        values (
            $1::uuid,
            $2::uuid,
            $3,
            $4,
            $5,
            coalesce(
                (select max(position) + 1 from gallery_images where project_id = $2::uuid),
                0
            )
        )
        returning id::text as id
        "#,
    )
    .bind(&image_id)
    .bind(&project_id)
    .bind(&storage_key)
    .bind(caption.trim())
    .bind(&content_type)
    .fetch_one(&pool)
    .await?;

    let id: String = row.get("id");
    Ok((StatusCode::CREATED, Json(json!({ "id": id }))).into_response())
}
