use axum::response::{IntoResponse, Response};
use axum::{extract::Path, extract::State, http::StatusCode};
use sqlx::Row;

use crate::error::AppError;
use crate::extract::AuthUser;
use crate::routes::owner::{ensure_not_in_review, require_project_owner};
use crate::storage::Storage;

pub async fn delete_version(
    State(pool): State<sqlx::PgPool>,
    State(storage): State<Storage>,
    AuthUser(user): AuthUser,
    Path((slug, version)): Path<(String, String)>,
) -> Result<Response, AppError> {
    let project_id = require_project_owner(&pool, &slug, &user.id).await?;
    ensure_not_in_review(&pool, &project_id).await?;

    let row = sqlx::query(
        r#"
        select v.id::text as id, f.storage_key
        from versions v
        left join files f on f.version_id = v.id
        where v.project_id = $1::uuid and v.version_number = $2
        "#,
    )
    .bind(&project_id)
    .bind(&version)
    .fetch_optional(&pool)
    .await?;

    let Some(row) = row else {
        return Err(AppError::not_found("version not found"));
    };

    let version_id: String = row.get("id");
    let storage_key: Option<String> = row.get("storage_key");

    sqlx::query("delete from versions where id = $1::uuid")
        .bind(&version_id)
        .execute(&pool)
        .await?;

    if let Some(storage_key) = storage_key {
        let _ = storage.delete(&storage_key).await;
    }

    Ok(StatusCode::NO_CONTENT.into_response())
}
