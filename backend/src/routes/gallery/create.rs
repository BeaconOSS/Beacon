use axum::response::{IntoResponse, Response};
use axum::{Json, extract::Multipart, extract::Path, extract::State, http::StatusCode};
use serde_json::json;
use sqlx::Row;

use crate::error::AppError;
use crate::extract::AuthUser;
use crate::routes::owner::{ensure_not_in_review, require_project_owner};
use crate::storage::Storage;

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
    mut multipart: Multipart,
) -> Result<Response, AppError> {
    let project_id = require_project_owner(&pool, &slug, &user.id).await?;
    ensure_not_in_review(&pool, &project_id).await?;

    let mut caption = String::new();
    let mut content_type = String::new();
    let mut image_bytes: Option<Vec<u8>> = None;

    loop {
        let field = match multipart.next_field().await {
            Ok(Some(field)) => field,
            Ok(None) => break,
            Err(_) => return Err(AppError::bad_request("invalid upload")),
        };

        match field.name() {
            Some("caption") => caption = field.text().await.unwrap_or_default(),
            Some("image") => {
                content_type = field
                    .content_type()
                    .map(|c| c.to_string())
                    .unwrap_or_default();
                match field.bytes().await {
                    Ok(bytes) => image_bytes = Some(bytes.to_vec()),
                    Err(_) => return Err(AppError::bad_request("invalid upload")),
                }
            }
            _ => {
                let _ = field.bytes().await;
            }
        }
    }

    let Some(extension) = ALLOWED_IMAGE_TYPES
        .iter()
        .find(|(mime, _)| *mime == content_type)
        .map(|(_, ext)| *ext)
    else {
        return Err(AppError::bad_request("an image file is required"));
    };

    let Some(bytes) = image_bytes else {
        return Err(AppError::bad_request("an image file is required"));
    };
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
