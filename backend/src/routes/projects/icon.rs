use axum::response::{IntoResponse, Response};
use axum::{
    Json, extract::Multipart, extract::Path, extract::Query, extract::State, http::StatusCode,
    http::header,
};
use axum_extra::extract::cookie::CookieJar;
use serde::Deserialize;
use serde_json::json;
use sqlx::Row;

use crate::error::AppError;
use crate::extract::AuthUser;
use crate::routes::owner::{ensure_not_in_review, require_project_owner};
use crate::session;
use crate::storage::Storage;

const ALLOWED_IMAGE_TYPES: [(&str, &str); 4] = [
    ("image/png", "png"),
    ("image/jpeg", "jpg"),
    ("image/webp", "webp"),
    ("image/gif", "gif"),
];

fn content_type_for_key(key: &str) -> &'static str {
    let ext = key.rsplit('.').next().unwrap_or("");
    match ext {
        "png" => "image/png",
        "jpg" => "image/jpeg",
        "webp" => "image/webp",
        "gif" => "image/gif",
        _ => "application/octet-stream",
    }
}

pub async fn upload_icon(
    State(pool): State<sqlx::PgPool>,
    State(storage): State<Storage>,
    AuthUser(user): AuthUser,
    Path(slug): Path<String>,
    mut multipart: Multipart,
) -> Result<Response, AppError> {
    let project_id = require_project_owner(&pool, &slug, &user.id).await?;
    ensure_not_in_review(&pool, &project_id).await?;

    let mut content_type = String::new();
    let mut image_bytes: Option<Vec<u8>> = None;

    loop {
        let field = match multipart.next_field().await {
            Ok(Some(field)) => field,
            Ok(None) => break,
            Err(_) => return Err(AppError::bad_request("invalid upload")),
        };

        if field.name() == Some("icon") {
            content_type = field
                .content_type()
                .map(|c| c.to_string())
                .unwrap_or_default();
            match field.bytes().await {
                Ok(bytes) => image_bytes = Some(bytes.to_vec()),
                Err(_) => return Err(AppError::bad_request("invalid upload")),
            }
        } else {
            let _ = field.bytes().await;
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

    let previous_key: Option<String> =
        sqlx::query("select icon_key from projects where id = $1::uuid")
            .bind(&project_id)
            .fetch_one(&pool)
            .await?
            .get("icon_key");

    let storage_key = format!("{project_id}/icon.{extension}");

    storage
        .put(&storage_key, &bytes, &content_type)
        .await
        .map_err(|_| AppError::internal("could not store icon"))?;

    if let Some(previous_key) = previous_key
        && previous_key != storage_key
    {
        let _ = storage.delete(&previous_key).await;
    }

    sqlx::query("update projects set icon_key = $1, updated_at = now() where id = $2::uuid")
        .bind(&storage_key)
        .bind(&project_id)
        .execute(&pool)
        .await?;

    sqlx::query(
        "update projects set status = 'in_review', submitted_at = now() \
         where id = $1::uuid and status = 'approved'",
    )
    .bind(&project_id)
    .execute(&pool)
    .await?;

    Ok((
        StatusCode::OK,
        Json(json!({ "icon_url": format!("/projects/{slug}/icon") })),
    )
        .into_response())
}

pub async fn delete_icon(
    State(pool): State<sqlx::PgPool>,
    State(storage): State<Storage>,
    AuthUser(user): AuthUser,
    Path(slug): Path<String>,
) -> Result<Response, AppError> {
    let project_id = require_project_owner(&pool, &slug, &user.id).await?;
    ensure_not_in_review(&pool, &project_id).await?;

    let key: Option<String> = sqlx::query("select icon_key from projects where id = $1::uuid")
        .bind(&project_id)
        .fetch_one(&pool)
        .await?
        .get("icon_key");

    if let Some(key) = key {
        let _ = storage.delete(&key).await;
        sqlx::query("update projects set icon_key = null, updated_at = now() where id = $1::uuid")
            .bind(&project_id)
            .execute(&pool)
            .await?;
        sqlx::query(
            "update projects set status = 'in_review', submitted_at = now() \
             where id = $1::uuid and status = 'approved'",
        )
        .bind(&project_id)
        .execute(&pool)
        .await?;
    }

    Ok(StatusCode::NO_CONTENT.into_response())
}

#[derive(Deserialize)]
pub struct IconQuery {
    revision: Option<String>,
}

pub async fn serve_icon(
    State(pool): State<sqlx::PgPool>,
    State(storage): State<Storage>,
    jar: CookieJar,
    Path(slug): Path<String>,
    Query(params): Query<IconQuery>,
) -> Result<Response, AppError> {
    let row = sqlx::query(
        "select owner_id::text as owner_id, icon_key, published_icon_key \
         from projects where slug = $1",
    )
    .bind(&slug)
    .fetch_optional(&pool)
    .await?;

    let Some(row) = row else {
        return Err(AppError::not_found("icon not found"));
    };

    let want_pending = params.revision.as_deref() == Some("pending");
    let key: Option<String> = if want_pending {
        let owner_id: String = row.get("owner_id");
        let viewer = match jar.get(session::SESSION_COOKIE) {
            Some(cookie) => session::lookup(&pool, cookie.value()).await.ok().flatten(),
            None => None,
        };
        let allowed = viewer.as_ref().is_some_and(|user| {
            user.id == owner_id
                || user.role == crate::constants::ROLE_MODERATOR
                || user.role == crate::constants::ROLE_ADMIN
        });
        if !allowed {
            return Err(AppError::not_found("icon not found"));
        }
        row.get("icon_key")
    } else {
        let published: Option<String> = row.get("published_icon_key");
        published.or_else(|| row.get("icon_key"))
    };

    let Some(key) = key else {
        return Err(AppError::not_found("icon not found"));
    };

    let bytes = storage
        .get(&key)
        .await
        .map_err(|_| AppError::internal("could not read icon"))?;

    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, content_type_for_key(&key))],
        bytes,
    )
        .into_response())
}
