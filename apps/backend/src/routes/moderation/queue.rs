use axum::response::{IntoResponse, Response};
use axum::{Json, extract::State, http::StatusCode};
use serde::Serialize;
use serde_json::json;
use sqlx::Row;

use crate::error::AppError;
use crate::extract::ModeratorUser;
use crate::routes::sql::created_at_utc;

#[derive(Serialize)]
struct QueueItem {
    id: String,
    slug: String,
    title: String,
    summary: String,
    project_type: String,
    owner: String,
    icon_url: Option<String>,
    submitted_at: Option<String>,
}

pub async fn queue(
    State(pool): State<sqlx::PgPool>,
    ModeratorUser(_): ModeratorUser,
) -> Result<Response, AppError> {
    let rows = sqlx::query(concat!(
        r#"
        select
            p.id::text as id,
            p.slug,
            p.title,
            p.summary,
            p.project_type,
            p.icon_key,
            u.username as owner,
            "#,
        created_at_utc!("p.submitted_at", "submitted_at"),
        r#"
        from projects p
        join users u on u.id = p.owner_id
        where p.status = 'in_review'
        order by p.submitted_at asc nulls last, p.created_at asc
        "#,
    ))
    .fetch_all(&pool)
    .await?;

    let items: Vec<QueueItem> = rows
        .into_iter()
        .map(|row| {
            let slug: String = row.get("slug");
            let icon_key: Option<String> = row.get("icon_key");
            QueueItem {
                icon_url: icon_key.map(|_| format!("/projects/{slug}/icon")),
                id: row.get("id"),
                title: row.get("title"),
                summary: row.get("summary"),
                project_type: row.get("project_type"),
                owner: row.get("owner"),
                submitted_at: row.get("submitted_at"),
                slug,
            }
        })
        .collect();

    Ok((StatusCode::OK, Json(json!({ "projects": items }))).into_response())
}
