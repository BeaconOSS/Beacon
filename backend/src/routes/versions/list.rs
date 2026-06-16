use axum::response::{IntoResponse, Response};
use axum::{Json, extract::Path, extract::State, http::StatusCode};
use serde::Serialize;
use serde_json::json;
use sqlx::Row;

use crate::error::AppError;

#[derive(Serialize)]
struct VersionFile {
    filename: String,
    size: i64,
    sha256: String,
}

#[derive(Serialize)]
struct Version {
    id: String,
    version_number: String,
    name: String,
    changelog: String,
    channel: String,
    download_count: i64,
    created_at: String,
    file: Option<VersionFile>,
}

pub async fn list_versions(
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

    let rows = sqlx::query(concat!(
        r#"
        select
            v.id::text as id,
            v.version_number,
            v.name,
            v.changelog,
            v.channel,
            v.download_count,
            "#,
        crate::routes::sql::created_at_utc!("v.created_at"),
        r#",
            f.filename as file_filename,
            f.size as file_size,
            f.sha256 as file_sha256
        from versions v
        left join files f on f.version_id = v.id
        where v.project_id = $1::uuid
        order by v.created_at desc
        "#,
    ))
    .bind(&project_id)
    .fetch_all(&pool)
    .await?;

    let versions: Vec<Version> = rows
        .into_iter()
        .map(|row| {
            let filename: Option<String> = row.get("file_filename");
            let file = filename.map(|filename| VersionFile {
                filename,
                size: row.get("file_size"),
                sha256: row.get("file_sha256"),
            });
            Version {
                id: row.get("id"),
                version_number: row.get("version_number"),
                name: row.get("name"),
                changelog: row.get("changelog"),
                channel: row.get("channel"),
                download_count: row.get("download_count"),
                created_at: row.get("created_at"),
                file,
            }
        })
        .collect();
    Ok((StatusCode::OK, Json(json!({ "versions": versions }))).into_response())
}
