use axum::response::{IntoResponse, Response};
use axum::{Json, extract::Path, extract::State, http::StatusCode};
use serde::Serialize;
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

#[derive(Serialize)]
struct ProjectDetail {
    id: String,
    slug: String,
    title: String,
    summary: String,
    description: String,
    project_type: String,
    download_count: i64,
    owner: String,
    created_at: String,
}

pub async fn list(State(pool): State<sqlx::PgPool>) -> Response {
    let rows = sqlx::query(
        r#"
        select
            id::text as id,
            slug,
            title,
            summary,
            project_type,
            download_count,
            to_char(created_at at time zone 'utc', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') as created_at
        from projects
        where published = true
        order by created_at desc
        "#,
    )
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

pub async fn detail(State(pool): State<sqlx::PgPool>, Path(slug): Path<String>) -> Response {
    let row = sqlx::query(
        r#"
        select
            p.id::text as id,
            p.slug,
            p.title,
            p.summary,
            p.description,
            p.project_type,
            p.download_count,
            u.username as owner,
            to_char(p.created_at at time zone 'utc', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') as created_at
        from projects p
        join users u on u.id = p.owner_id
        where p.slug = $1 and p.published = true
        "#,
    )
    .bind(&slug)
    .fetch_optional(&pool)
    .await;

    match row {
        Ok(Some(row)) => {
            let project = ProjectDetail {
                id: row.get("id"),
                slug: row.get("slug"),
                title: row.get("title"),
                summary: row.get("summary"),
                description: row.get("description"),
                project_type: row.get("project_type"),
                download_count: row.get("download_count"),
                owner: row.get("owner"),
                created_at: row.get("created_at"),
            };
            (StatusCode::OK, Json(project)).into_response()
        }
        Ok(None) => error(StatusCode::NOT_FOUND, "project not found").into_response(),
        Err(_) => {
            error(StatusCode::INTERNAL_SERVER_ERROR, "could not load project").into_response()
        }
    }
}
