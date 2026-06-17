use axum::response::{IntoResponse, Response};
use axum::{Json, extract::Path, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::Row;

use crate::error::AppError;
use crate::extract::ModeratorUser;
use crate::routes::sql::created_at_utc;

#[derive(Serialize)]
struct ModeratorNote {
    id: String,
    author: String,
    body: String,
    created_at: Option<String>,
}

async fn project_id_for_slug(pool: &sqlx::PgPool, slug: &str) -> Result<String, AppError> {
    let row = sqlx::query("select id::text as id from projects where slug = $1")
        .bind(slug)
        .fetch_optional(pool)
        .await?;
    match row {
        Some(row) => Ok(row.get("id")),
        None => Err(AppError::not_found("project not found")),
    }
}

pub async fn list_moderator_notes(
    State(pool): State<sqlx::PgPool>,
    ModeratorUser(_): ModeratorUser,
    Path(slug): Path<String>,
) -> Result<Response, AppError> {
    let project_id = project_id_for_slug(&pool, &slug).await?;

    let rows = sqlx::query(concat!(
        r#"
        select
            n.id::text as id,
            u.username as author,
            n.body,
            "#,
        created_at_utc!("n.created_at"),
        r#"
        from project_moderator_notes n
        join users u on u.id = n.author_id
        where n.project_id = $1::uuid
        order by n.created_at desc
        "#,
    ))
    .bind(&project_id)
    .fetch_all(&pool)
    .await?;

    let notes: Vec<ModeratorNote> = rows
        .into_iter()
        .map(|row| ModeratorNote {
            id: row.get("id"),
            author: row.get("author"),
            body: row.get("body"),
            created_at: row.get("created_at"),
        })
        .collect();

    Ok((StatusCode::OK, Json(json!({ "notes": notes }))).into_response())
}

#[derive(Deserialize)]
pub struct AddNoteRequest {
    #[serde(default)]
    body: String,
}

pub async fn add_moderator_note(
    State(pool): State<sqlx::PgPool>,
    ModeratorUser(moderator): ModeratorUser,
    Path(slug): Path<String>,
    Json(payload): Json<AddNoteRequest>,
) -> Result<Response, AppError> {
    let body = payload.body.trim();
    if body.is_empty() {
        return Err(AppError::bad_request("note body cannot be empty"));
    }

    let project_id = project_id_for_slug(&pool, &slug).await?;

    let row = sqlx::query(concat!(
        r#"
        with inserted as (
            insert into project_moderator_notes (project_id, author_id, body)
            values ($1::uuid, $2::uuid, $3)
            returning id, author_id, body, created_at
        )
        select
            inserted.id::text as id,
            u.username as author,
            inserted.body,
            "#,
        created_at_utc!("inserted.created_at"),
        r#"
        from inserted
        join users u on u.id = inserted.author_id
        "#,
    ))
    .bind(&project_id)
    .bind(&moderator.id)
    .bind(body)
    .fetch_one(&pool)
    .await?;

    let note = ModeratorNote {
        id: row.get("id"),
        author: row.get("author"),
        body: row.get("body"),
        created_at: row.get("created_at"),
    };

    Ok((StatusCode::CREATED, Json(note)).into_response())
}
