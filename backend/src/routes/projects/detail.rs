use axum::response::{IntoResponse, Response};
use axum::{Json, extract::Path, extract::State, http::StatusCode};
use serde::Serialize;
use sqlx::Row;

use crate::error::error;

#[derive(Serialize)]
struct CategoryTag {
    slug: String,
    name: String,
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
    categories: Vec<CategoryTag>,
    created_at: String,
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
            let id: String = row.get("id");

            let category_rows = sqlx::query(
                r#"
                select c.slug, c.name
                from project_categories pc
                join categories c on c.id = pc.category_id
                where pc.project_id = $1::uuid
                order by c.ordering
                "#,
            )
            .bind(&id)
            .fetch_all(&pool)
            .await;

            let categories = match category_rows {
                Ok(rows) => rows
                    .into_iter()
                    .map(|row| CategoryTag {
                        slug: row.get("slug"),
                        name: row.get("name"),
                    })
                    .collect(),
                Err(_) => {
                    return error(StatusCode::INTERNAL_SERVER_ERROR, "could not load project")
                        .into_response();
                }
            };

            let project = ProjectDetail {
                id,
                slug: row.get("slug"),
                title: row.get("title"),
                summary: row.get("summary"),
                description: row.get("description"),
                project_type: row.get("project_type"),
                download_count: row.get("download_count"),
                owner: row.get("owner"),
                categories,
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
