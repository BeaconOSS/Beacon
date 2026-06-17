use axum::response::{IntoResponse, Response};
use axum::{extract::Path, extract::State, http::StatusCode};
use sqlx::Row;

use crate::error::AppError;
use crate::extract::AuthUser;
use crate::routes::owner::require_project_owner;
use crate::storage::Storage;

pub async fn delete_project(
    State(pool): State<sqlx::PgPool>,
    State(storage): State<Storage>,
    AuthUser(user): AuthUser,
    Path(slug): Path<String>,
) -> Result<Response, AppError> {
    let project_id = require_project_owner(&pool, &slug, &user.id).await?;

    let mut storage_keys: Vec<String> = Vec::new();

    let icon = sqlx::query("select icon_key from projects where id = $1::uuid")
        .bind(&project_id)
        .fetch_optional(&pool)
        .await?;
    if let Some(row) = icon
        && let Some(key) = row.get::<Option<String>, _>("icon_key")
    {
        storage_keys.push(key);
    }

    let file_rows = sqlx::query(
        "select f.storage_key \
         from files f \
         join versions v on v.id = f.version_id \
         where v.project_id = $1::uuid",
    )
    .bind(&project_id)
    .fetch_all(&pool)
    .await?;
    for row in file_rows {
        storage_keys.push(row.get("storage_key"));
    }

    let gallery_rows =
        sqlx::query("select storage_key from gallery_images where project_id = $1::uuid")
            .bind(&project_id)
            .fetch_all(&pool)
            .await?;
    for row in gallery_rows {
        storage_keys.push(row.get("storage_key"));
    }

    sqlx::query("delete from projects where id = $1::uuid")
        .bind(&project_id)
        .execute(&pool)
        .await?;

    for key in storage_keys {
        let _ = storage.delete(&key).await;
    }

    Ok(StatusCode::NO_CONTENT.into_response())
}
