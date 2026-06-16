use axum::response::{IntoResponse, Response};
use axum::{Json, extract::Multipart, extract::Path, extract::State, http::StatusCode};
use serde_json::json;
use sha2::{Digest, Sha256};
use sqlx::Row;

use crate::error::AppError;
use crate::extract::AuthUser;
use crate::routes::owner::require_project_owner;
use crate::storage::Storage;

const VERSION_CHANNELS: [&str; 3] = ["release", "beta", "alpha"];

pub async fn create_version(
    State(pool): State<sqlx::PgPool>,
    State(storage): State<Storage>,
    AuthUser(user): AuthUser,
    Path(slug): Path<String>,
    mut multipart: Multipart,
) -> Result<Response, AppError> {
    let project_id = require_project_owner(&pool, &slug, &user.id).await?;

    let mut version_number = String::new();
    let mut name = String::new();
    let mut changelog = String::new();
    let mut channel = String::new();
    let mut filename = String::new();
    let mut file_bytes: Option<Vec<u8>> = None;

    loop {
        let field = match multipart.next_field().await {
            Ok(Some(field)) => field,
            Ok(None) => break,
            Err(_) => return Err(AppError::bad_request("invalid upload")),
        };

        match field.name() {
            Some("version_number") => version_number = field.text().await.unwrap_or_default(),
            Some("name") => name = field.text().await.unwrap_or_default(),
            Some("changelog") => changelog = field.text().await.unwrap_or_default(),
            Some("channel") => channel = field.text().await.unwrap_or_default(),
            Some("file") => {
                filename = field.file_name().map(|f| f.to_string()).unwrap_or_default();
                match field.bytes().await {
                    Ok(bytes) => file_bytes = Some(bytes.to_vec()),
                    Err(_) => return Err(AppError::bad_request("invalid upload")),
                }
            }
            _ => {
                let _ = field.bytes().await;
            }
        }
    }

    let version_number = version_number.trim().to_string();
    if version_number.is_empty() {
        return Err(AppError::bad_request("a version number is required"));
    }

    if channel.is_empty() {
        channel = "release".to_string();
    }
    if !VERSION_CHANNELS.contains(&channel.as_str()) {
        return Err(AppError::bad_request("invalid channel"));
    }

    let safe_filename = sanitize_filename(&filename);
    if safe_filename.is_empty() {
        return Err(AppError::bad_request("a file is required"));
    }

    let Some(bytes) = file_bytes else {
        return Err(AppError::bad_request("a file is required"));
    };
    if bytes.is_empty() {
        return Err(AppError::bad_request("a file is required"));
    }

    let size = bytes.len() as i64;
    let sha256 = hex_encode(&Sha256::digest(&bytes));
    let storage_key = format!("{project_id}/{version_number}/{safe_filename}");

    storage
        .put(&storage_key, &bytes, "application/octet-stream")
        .await
        .map_err(|_| AppError::internal("could not store file"))?;

    let row = sqlx::query(
        r#"
        with new_version as (
            insert into versions (project_id, version_number, name, changelog, channel)
            values ($1::uuid, $2, $3, $4, $5)
            returning id
        ), new_file as (
            insert into files (version_id, filename, size, sha256, storage_key)
            select id, $6, $7, $8, $9 from new_version
        )
        select id::text as id from new_version
        "#,
    )
    .bind(&project_id)
    .bind(&version_number)
    .bind(name.trim())
    .bind(changelog.trim())
    .bind(&channel)
    .bind(&safe_filename)
    .bind(size)
    .bind(&sha256)
    .bind(&storage_key)
    .fetch_one(&pool)
    .await;

    let row = match row {
        Ok(row) => row,
        Err(sqlx::Error::Database(db)) if db.code().as_deref() == Some("23505") => {
            return Err(AppError::conflict("version already exists"));
        }
        Err(e) => return Err(e.into()),
    };

    sqlx::query(
        "update projects set status = 'in_review', submitted_at = now(), updated_at = now() \
         where id = $1::uuid and status = 'approved'",
    )
    .bind(&project_id)
    .execute(&pool)
    .await?;

    let id: String = row.get("id");
    Ok((
        StatusCode::CREATED,
        Json(json!({ "id": id, "version_number": version_number })),
    )
        .into_response())
}

fn sanitize_filename(filename: &str) -> String {
    let base = filename.rsplit(['/', '\\']).next().unwrap_or(filename);
    base.chars()
        .filter(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '.' | '-' | '_'))
        .collect()
}

fn hex_encode(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push_str(&format!("{byte:02x}"));
    }
    out
}
