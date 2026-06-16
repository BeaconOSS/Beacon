use axum::response::{IntoResponse, Response};
use axum::{extract::Path, extract::Query, extract::State, http::StatusCode, http::header};
use serde::Deserialize;
use sqlx::Row;

use crate::error::AppError;
use crate::extract::ModeratorUser;
use crate::pack;
use crate::storage::Storage;

#[derive(Deserialize)]
pub struct InnerFileQuery {
    path: String,
}

pub async fn inner_file(
    State(pool): State<sqlx::PgPool>,
    State(storage): State<Storage>,
    ModeratorUser(_): ModeratorUser,
    Path((slug, version_number)): Path<(String, String)>,
    Query(params): Query<InnerFileQuery>,
) -> Result<Response, AppError> {
    if params.path.trim().is_empty() {
        return Err(AppError::bad_request("a file path is required"));
    }

    let row = sqlx::query(
        r#"
        select f.storage_key as storage_key
        from versions v
        join projects p on p.id = v.project_id
        join files f on f.version_id = v.id and f.is_primary = true
        where p.slug = $1 and v.version_number = $2
        "#,
    )
    .bind(&slug)
    .bind(&version_number)
    .fetch_optional(&pool)
    .await?;

    let Some(row) = row else {
        return Err(AppError::not_found("version not found"));
    };
    let storage_key: String = row.get("storage_key");

    let bytes = storage
        .get(&storage_key)
        .await
        .map_err(|_| AppError::internal("could not read file"))?;

    let inner_path = params.path.clone();
    let extracted = tokio::task::spawn_blocking(move || pack::read_inner_file(&bytes, &inner_path))
        .await
        .map_err(|_| AppError::internal("could not read archive"))?;

    match extracted {
        Ok(Some(data)) => {
            let content_type = pack::guess_content_type(&params.path);
            Ok((
                StatusCode::OK,
                [(header::CONTENT_TYPE, content_type.to_string())],
                data,
            )
                .into_response())
        }
        Ok(None) => Err(AppError::not_found("file not found in archive")),
        Err(message) => Err(AppError::bad_request(&message)),
    }
}
