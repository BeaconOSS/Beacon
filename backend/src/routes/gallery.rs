use axum::response::{IntoResponse, Response};
use axum::routing::{delete, get, post};
use axum::{Json, Router, extract::Multipart, extract::Path, extract::State, http::StatusCode, http::header};
use axum_extra::extract::cookie::CookieJar;
use serde::Serialize;
use serde_json::json;
use sqlx::Row;

use crate::error::error;
use crate::session;
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
    jar: CookieJar,
    Path(slug): Path<String>,
    mut multipart: Multipart,
) -> Response {
    let Some(token) = jar.get(session::SESSION_COOKIE).map(|c| c.value().to_string()) else {
        return error(StatusCode::UNAUTHORIZED, "not signed in").into_response();
    };

    let user_id = match session::lookup(&pool, &token).await {
        Ok(Some(user)) => user.id,
        Ok(None) => return error(StatusCode::UNAUTHORIZED, "not signed in").into_response(),
        Err(_) => {
            return error(StatusCode::INTERNAL_SERVER_ERROR, "could not read session")
                .into_response();
        }
    };

    let project = sqlx::query(
        "select id::text as id, owner_id::text as owner_id from projects where slug = $1",
    )
    .bind(&slug)
    .fetch_optional(&pool)
    .await;

    let (project_id, owner_id) = match project {
        Ok(Some(row)) => {
            let id: String = row.get("id");
            let owner_id: String = row.get("owner_id");
            (id, owner_id)
        }
        Ok(None) => return error(StatusCode::NOT_FOUND, "project not found").into_response(),
        Err(_) => {
            return error(StatusCode::INTERNAL_SERVER_ERROR, "could not load project")
                .into_response();
        }
    };

    if owner_id != user_id {
        return error(StatusCode::FORBIDDEN, "not your project").into_response();
    }

    let mut caption = String::new();
    let mut content_type = String::new();
    let mut image_bytes: Option<Vec<u8>> = None;

    loop {
        let field = match multipart.next_field().await {
            Ok(Some(field)) => field,
            Ok(None) => break,
            Err(_) => return error(StatusCode::BAD_REQUEST, "invalid upload").into_response(),
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
                    Err(_) => {
                        return error(StatusCode::BAD_REQUEST, "invalid upload").into_response();
                    }
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
        return error(StatusCode::BAD_REQUEST, "an image file is required").into_response();
    };

    let Some(bytes) = image_bytes else {
        return error(StatusCode::BAD_REQUEST, "an image file is required").into_response();
    };
    if bytes.is_empty() {
        return error(StatusCode::BAD_REQUEST, "an image file is required").into_response();
    }

    let image_id = match sqlx::query("select gen_random_uuid()::text as id")
        .fetch_one(&pool)
        .await
    {
        Ok(row) => row.get::<String, _>("id"),
        Err(_) => {
            return error(StatusCode::INTERNAL_SERVER_ERROR, "could not save image").into_response();
        }
    };
    let storage_key = format!("{project_id}/gallery/{image_id}.{extension}");

    if storage.put(&storage_key, &bytes, &content_type).await.is_err() {
        return error(StatusCode::INTERNAL_SERVER_ERROR, "could not store image").into_response();
    }

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
    .await;

    match row {
        Ok(row) => {
            let id: String = row.get("id");
            (StatusCode::CREATED, Json(json!({ "id": id }))).into_response()
        }
        Err(_) => {
            error(StatusCode::INTERNAL_SERVER_ERROR, "could not save image").into_response()
        }
    }
}

async fn list_gallery_images(
    State(pool): State<sqlx::PgPool>,
    Path(slug): Path<String>,
) -> Response {
    let project = sqlx::query(
        "select id::text as id from projects where slug = $1 and published = true",
    )
    .bind(&slug)
    .fetch_optional(&pool)
    .await;

    let project_id: String = match project {
        Ok(Some(row)) => row.get("id"),
        Ok(None) => return error(StatusCode::NOT_FOUND, "project not found").into_response(),
        Err(_) => {
            return error(StatusCode::INTERNAL_SERVER_ERROR, "could not load gallery")
                .into_response();
        }
    };

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
    .await;

    match rows {
        Ok(rows) => {
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
            (StatusCode::OK, Json(json!({ "images": images }))).into_response()
        }
        Err(_) => {
            error(StatusCode::INTERNAL_SERVER_ERROR, "could not load gallery").into_response()
        }
    }
}

async fn serve_gallery_image(
    State(pool): State<sqlx::PgPool>,
    State(storage): State<Storage>,
    Path((slug, image_id)): Path<(String, String)>,
) -> Response {
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
    .await;

    let (storage_key, content_type) = match row {
        Ok(Some(row)) => {
            let storage_key: String = row.get("storage_key");
            let content_type: String = row.get("content_type");
            (storage_key, content_type)
        }
        Ok(None) => return error(StatusCode::NOT_FOUND, "image not found").into_response(),
        Err(_) => {
            return error(StatusCode::INTERNAL_SERVER_ERROR, "could not load image")
                .into_response();
        }
    };

    let bytes = match storage.get(&storage_key).await {
        Ok(bytes) => bytes,
        Err(_) => {
            return error(StatusCode::INTERNAL_SERVER_ERROR, "could not read image")
                .into_response();
        }
    };

    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, content_type)],
        bytes,
    )
        .into_response()
}

async fn delete_gallery_image(
    State(pool): State<sqlx::PgPool>,
    State(storage): State<Storage>,
    jar: CookieJar,
    Path((slug, image_id)): Path<(String, String)>,
) -> Response {
    let Some(token) = jar.get(session::SESSION_COOKIE).map(|c| c.value().to_string()) else {
        return error(StatusCode::UNAUTHORIZED, "not signed in").into_response();
    };

    let user_id = match session::lookup(&pool, &token).await {
        Ok(Some(user)) => user.id,
        Ok(None) => return error(StatusCode::UNAUTHORIZED, "not signed in").into_response(),
        Err(_) => {
            return error(StatusCode::INTERNAL_SERVER_ERROR, "could not read session")
                .into_response();
        }
    };

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
    .await;

    let (storage_key, owner_id) = match row {
        Ok(Some(row)) => {
            let storage_key: String = row.get("storage_key");
            let owner_id: String = row.get("owner_id");
            (storage_key, owner_id)
        }
        Ok(None) => return error(StatusCode::NOT_FOUND, "image not found").into_response(),
        Err(_) => {
            return error(StatusCode::INTERNAL_SERVER_ERROR, "could not load image")
                .into_response();
        }
    };

    if owner_id != user_id {
        return error(StatusCode::FORBIDDEN, "not your project").into_response();
    }

    if sqlx::query("delete from gallery_images where id = $1::uuid")
        .bind(&image_id)
        .execute(&pool)
        .await
        .is_err()
    {
        return error(StatusCode::INTERNAL_SERVER_ERROR, "could not delete image").into_response();
    }

    let _ = storage.delete(&storage_key).await;

    StatusCode::NO_CONTENT.into_response()
}
