use axum::response::{IntoResponse, Response};
use axum::{Json, extract::Multipart, extract::Path, extract::State, http::StatusCode, http::header};
use axum_extra::extract::cookie::CookieJar;
use serde::Serialize;
use serde_json::json;
use sha2::{Digest, Sha256};
use sqlx::Row;

use crate::error::error;
use crate::session;
use crate::storage::Storage;

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

const VERSION_CHANNELS: [&str; 3] = ["release", "beta", "alpha"];

pub async fn list_versions(
    State(pool): State<sqlx::PgPool>,
    Path(slug): Path<String>,
) -> Response {
    let project = sqlx::query("select id::text as id from projects where slug = $1 and published = true")
        .bind(&slug)
        .fetch_optional(&pool)
        .await;

    let project_id: String = match project {
        Ok(Some(row)) => row.get("id"),
        Ok(None) => return error(StatusCode::NOT_FOUND, "project not found").into_response(),
        Err(_) => {
            return error(StatusCode::INTERNAL_SERVER_ERROR, "could not load versions")
                .into_response();
        }
    };

    let rows = sqlx::query(
        r#"
        select
            v.id::text as id,
            v.version_number,
            v.name,
            v.changelog,
            v.channel,
            v.download_count,
            to_char(v.created_at at time zone 'utc', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') as created_at,
            f.filename as file_filename,
            f.size as file_size,
            f.sha256 as file_sha256
        from versions v
        left join files f on f.version_id = v.id
        where v.project_id = $1::uuid
        order by v.created_at desc
        "#,
    )
    .bind(&project_id)
    .fetch_all(&pool)
    .await;

    match rows {
        Ok(rows) => {
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
            (StatusCode::OK, Json(json!({ "versions": versions }))).into_response()
        }
        Err(_) => {
            error(StatusCode::INTERNAL_SERVER_ERROR, "could not load versions").into_response()
        }
    }
}

pub async fn download_version(
    State(pool): State<sqlx::PgPool>,
    State(storage): State<Storage>,
    Path((slug, version_number)): Path<(String, String)>,
) -> Response {
    let row = sqlx::query(
        r#"
        select
            v.id::text as version_id,
            f.filename as filename,
            f.storage_key as storage_key
        from versions v
        join projects p on p.id = v.project_id
        join files f on f.version_id = v.id
        where p.slug = $1 and p.published = true and v.version_number = $2
        "#,
    )
    .bind(&slug)
    .bind(&version_number)
    .fetch_optional(&pool)
    .await;

    let (version_id, filename, storage_key) = match row {
        Ok(Some(row)) => {
            let version_id: String = row.get("version_id");
            let filename: String = row.get("filename");
            let storage_key: String = row.get("storage_key");
            (version_id, filename, storage_key)
        }
        Ok(None) => return error(StatusCode::NOT_FOUND, "version not found").into_response(),
        Err(_) => {
            return error(StatusCode::INTERNAL_SERVER_ERROR, "could not load version")
                .into_response();
        }
    };

    let bytes = match storage.get(&storage_key).await {
        Ok(bytes) => bytes,
        Err(_) => {
            return error(StatusCode::INTERNAL_SERVER_ERROR, "could not read file").into_response();
        }
    };

    let _ = sqlx::query("update versions set download_count = download_count + 1 where id = $1::uuid")
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

    let disposition = format!("attachment; filename=\"{filename}\"");
    (
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, "application/octet-stream".to_string()),
            (header::CONTENT_DISPOSITION, disposition),
        ],
        bytes,
    )
        .into_response()
}

pub async fn create_version(
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
            Err(_) => return error(StatusCode::BAD_REQUEST, "invalid upload").into_response(),
        };

        match field.name() {
            Some("version_number") => version_number = field.text().await.unwrap_or_default(),
            Some("name") => name = field.text().await.unwrap_or_default(),
            Some("changelog") => changelog = field.text().await.unwrap_or_default(),
            Some("channel") => channel = field.text().await.unwrap_or_default(),
            Some("file") => {
                filename = field
                    .file_name()
                    .map(|f| f.to_string())
                    .unwrap_or_default();
                match field.bytes().await {
                    Ok(bytes) => file_bytes = Some(bytes.to_vec()),
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

    let version_number = version_number.trim().to_string();
    if version_number.is_empty() {
        return error(StatusCode::BAD_REQUEST, "a version number is required").into_response();
    }

    if channel.is_empty() {
        channel = "release".to_string();
    }
    if !VERSION_CHANNELS.contains(&channel.as_str()) {
        return error(StatusCode::BAD_REQUEST, "invalid channel").into_response();
    }

    let safe_filename = sanitize_filename(&filename);
    if safe_filename.is_empty() {
        return error(StatusCode::BAD_REQUEST, "a file is required").into_response();
    }

    let Some(bytes) = file_bytes else {
        return error(StatusCode::BAD_REQUEST, "a file is required").into_response();
    };
    if bytes.is_empty() {
        return error(StatusCode::BAD_REQUEST, "a file is required").into_response();
    }

    let size = bytes.len() as i64;
    let sha256 = hex_encode(&Sha256::digest(&bytes));
    let storage_key = format!("{project_id}/{version_number}/{safe_filename}");

    if storage
        .put(&storage_key, &bytes, "application/octet-stream")
        .await
        .is_err()
    {
        return error(StatusCode::INTERNAL_SERVER_ERROR, "could not store file").into_response();
    }

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

    match row {
        Ok(row) => {
            let id: String = row.get("id");
            (
                StatusCode::CREATED,
                Json(json!({ "id": id, "version_number": version_number })),
            )
                .into_response()
        }
        Err(sqlx::Error::Database(db)) if db.code().as_deref() == Some("23505") => {
            error(StatusCode::CONFLICT, "version already exists").into_response()
        }
        Err(_) => {
            error(StatusCode::INTERNAL_SERVER_ERROR, "could not create version").into_response()
        }
    }
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
