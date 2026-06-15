use axum::response::{IntoResponse, Response};
use axum::{Json, extract::Query, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::Row;

use crate::error::error;

#[derive(Serialize)]
struct Project {
    id: String,
    slug: String,
    title: String,
    summary: String,
    project_type: String,
    download_count: i64,
    created_at: String,
}

#[derive(Deserialize)]
pub struct ListQuery {
    category: Option<String>,
    q: Option<String>,
}

pub async fn list(State(pool): State<sqlx::PgPool>, Query(query): Query<ListQuery>) -> Response {
    let search = query
        .q
        .as_deref()
        .map(str::trim)
        .filter(|q| !q.is_empty())
        .map(|q| format!("%{q}%"));

    let rows = sqlx::query(
        r#"
        select
            p.id::text as id,
            p.slug,
            p.title,
            p.summary,
            p.project_type,
            p.download_count,
            to_char(p.created_at at time zone 'utc', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') as created_at
        from projects p
        where p.published = true
          and (
            $1::text is null
            or exists (
                select 1
                from project_categories pc
                join categories c on c.id = pc.category_id
                where pc.project_id = p.id and c.slug = $1
            )
          )
          and (
            $2::text is null
            or p.title ilike $2
            or p.summary ilike $2
          )
        order by p.created_at desc
        "#,
    )
    .bind(query.category.as_deref())
    .bind(search.as_deref())
    .fetch_all(&pool)
    .await;

    match rows {
        Ok(rows) => {
            let projects: Vec<Project> = rows
                .into_iter()
                .map(|row| Project {
                    id: row.get("id"),
                    slug: row.get("slug"),
                    title: row.get("title"),
                    summary: row.get("summary"),
                    project_type: row.get("project_type"),
                    download_count: row.get("download_count"),
                    created_at: row.get("created_at"),
                })
                .collect();
            (StatusCode::OK, Json(json!({ "projects": projects }))).into_response()
        }
        Err(_) => {
            error(StatusCode::INTERNAL_SERVER_ERROR, "could not load projects").into_response()
        }
    }
}
