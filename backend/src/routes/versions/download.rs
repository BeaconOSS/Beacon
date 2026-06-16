use axum::response::{IntoResponse, Response};
use axum::{extract::Path, extract::State, http::StatusCode, http::header};
use sqlx::Row;

use crate::error::AppError;
use crate::storage::Storage;

pub async fn download_version(
    State(pool): State<sqlx::PgPool>,
    State(storage): State<Storage>,
    Path((slug, version_number)): Path<(String, String)>,
) -> Result<Response, AppError> {
    let row = sqlx::query(
        r#"
        select
            v.id::text as version_id,
            p.id::text as project_id,
            f.filename as filename,
            f.storage_key as storage_key
        from versions v
        join projects p on p.id = v.project_id
        join files f on f.version_id = v.id
        where p.slug = $1 and p.status = 'approved' and p.visibility <> 'private'
          and v.version_number = $2
        "#,
    )
    .bind(&slug)
    .bind(&version_number)
    .fetch_optional(&pool)
    .await?;

    let Some(row) = row else {
        return Err(AppError::not_found("version not found"));
    };
    let version_id: String = row.get("version_id");
    let project_id: String = row.get("project_id");
    let filename: String = row.get("filename");
    let storage_key: String = row.get("storage_key");

    let bytes = storage
        .get(&storage_key)
        .await
        .map_err(|_| AppError::internal("could not read file"))?;

    let _ =
        sqlx::query("update versions set download_count = download_count + 1 where id = $1::uuid")
            .bind(&version_id)
            .execute(&pool)
            .await;

    let _ = sqlx::query(
        "update projects set download_count = download_count + 1 \
         where id = (select project_id from versions where id = $1::uuid)",
    )
    .bind(&version_id)
    .execute(&pool)
    .await;

    let _ = sqlx::query(
        "insert into project_daily_stats (project_id, downloads) values ($1::uuid, 1) \
         on conflict (project_id, day) \
         do update set downloads = project_daily_stats.downloads + 1",
    )
    .bind(&project_id)
    .execute(&pool)
    .await;

    let disposition = format!("attachment; filename=\"{filename}\"");
    Ok((
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, "application/octet-stream".to_string()),
            (header::CONTENT_DISPOSITION, disposition),
        ],
        bytes,
    )
        .into_response())
}
