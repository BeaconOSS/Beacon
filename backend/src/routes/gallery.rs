use axum::response::{IntoResponse, Response};
use axum::routing::{delete, get, post};
use axum::{Json, Router, extract::Multipart, extract::Path, extract::State, http::StatusCode, http::header};
use serde::Serialize;
use serde_json::json;
use sqlx::Row;

use crate::error::AppError;
use crate::extract::AuthUser;
use crate::state::AppState;
use crate::storage::Storage;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/projects/{slug}/gallery", get(list_gallery_images))
        .route("/projects/{slug}/gallery", post(create_gallery_image))
        .route("/projects/{slug}/gallery/{image}", get(serve_gallery_image))
        .route("/projects/{slug}/gallery/{image}", delete(delete_gallery_image))
}

#[derive(Serialize)]
struct GalleryImage {
    id: String,
    caption: String,
    url: String,
}

const ALLOWED_IMAGE_TYPES: [(&str, &str); 4] = [
    ("image/png", "png"),
    ("image/jpeg", "jpg"),
    ("image/webp", "webp"),
    ("image/gif", "gif"),
];

async fn create_gallery_image(
    State(pool): State<sqlx::PgPool>,
    State(storage): State<Storage>,
    AuthUser(user): AuthUser,
    Path(slug): Path<String>,
    mut multipart: Multipart,
) -> Result<Response, AppError> {
    let user_id = user.id;

    let project = sqlx::query(
        "select id::text as id, owner_id::text as owner_id from projects where slug = $1",
    )
    .bind(&slug)
    .fetch_optional(&pool)
    .await?;

    let Some(project) = project else {
        return Err(AppError::not_found("project not found"));
    };
    let project_id: String = project.get("id");
    let owner_id: String = project.get("owner_id");

    if owner_id != user_id {
        return Err(AppError::forbidden("not your project"));
    }

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

async fn list_gallery_images(
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

async fn serve_gallery_image(
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

    Ok((StatusCode::OK, [(header::CONTENT_TYPE, content_type)], bytes).into_response())
}

async fn delete_gallery_image(
    State(pool): State<sqlx::PgPool>,
    State(storage): State<Storage>,
    AuthUser(user): AuthUser,
    Path((slug, image_id)): Path<(String, String)>,
) -> Result<Response, AppError> {
    let user_id = user.id;

    let row = sqlx::query(
        r#"
        select g.id::text as id, g.storage_key, p.owner_id::text as owner_id
        from gallery_images g
        join projects p on p.id = g.project_id
        where p.slug = $1 and g.id = $2::uuid
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
    let owner_id: String = row.get("owner_id");

    if owner_id != user_id {
        return Err(AppError::forbidden("not your project"));
    }

    sqlx::query("delete from gallery_images where id = $1::uuid")
        .bind(&image_id)
        .execute(&pool)
        .await?;

    let _ = storage.delete(&storage_key).await;

    Ok(StatusCode::NO_CONTENT.into_response())
}
