use axum::response::{IntoResponse, Response};
use axum::{Json, extract::Query, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::Row;

use crate::error::AppError;

#[derive(Serialize)]
struct CategoryTag {
    slug: String,
    name: String,
}

#[derive(Serialize)]
struct Project {
    id: String,
    slug: String,
    title: String,
    summary: String,
    project_type: String,
    download_count: i64,
    icon_url: Option<String>,
    owner: String,
    categories: Vec<CategoryTag>,
    created_at: String,
    updated_at: String,
}

#[derive(Deserialize)]
pub struct ListQuery {
    category: Option<String>,
    q: Option<String>,
}

pub async fn list(
    State(pool): State<sqlx::PgPool>,
    Query(query): Query<ListQuery>,
) -> Result<Response, AppError> {
    let search = query
        .q
        .as_deref()
        .map(str::trim)
        .filter(|q| !q.is_empty())
        .map(|q| format!("%{q}%"));

    let rows = sqlx::query(concat!(
        r#"
        select
            p.id::text as id,
            p.slug,
            p.published_title as title,
            p.published_summary as summary,
            p.project_type,
            p.download_count,
            p.published_icon_key as icon_key,
            u.username as owner,
            "#,
        crate::routes::sql::created_at_utc!("p.created_at"),
        r#",
            to_char(p.updated_at at time zone 'utc', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') as updated_at
        from projects p
        join users u on u.id = p.owner_id
        where p.published_at is not null
          and p.visibility = 'public'
          and (
            $1::text is null
            or exists (
                select 1
                from project_published_categories pc
                join categories c on c.id = pc.category_id
                where pc.project_id = p.id and c.slug = $1
            )
          )
          and (
            $2::text is null
            or p.published_title ilike $2
            or p.published_summary ilike $2
          )
        order by p.published_at desc
        "#,
    ))
    .bind(query.category.as_deref())
    .bind(search.as_deref())
    .fetch_all(&pool)
    .await?;

    let ids: Vec<String> = rows.iter().map(|row| row.get("id")).collect();

    let category_rows = sqlx::query(
        r#"
        select
            pc.project_id::text as project_id,
            c.slug,
            c.name
        from project_published_categories pc
        join categories c on c.id = pc.category_id
        where pc.project_id = any($1::uuid[])
        order by c.ordering
        "#,
    )
    .bind(&ids)
    .fetch_all(&pool)
    .await?;

    let mut categories_by_project: std::collections::HashMap<String, Vec<CategoryTag>> =
        std::collections::HashMap::new();
    for row in category_rows {
        let project_id: String = row.get("project_id");
        categories_by_project
            .entry(project_id)
            .or_default()
            .push(CategoryTag {
                slug: row.get("slug"),
                name: row.get("name"),
            });
    }

    let projects: Vec<Project> = rows
        .into_iter()
        .map(|row| {
            let id: String = row.get("id");
            let slug: String = row.get("slug");
            let icon_key: Option<String> = row.get("icon_key");
            let icon_url = icon_key.map(|_| format!("/projects/{slug}/icon"));
            let categories = categories_by_project.remove(&id).unwrap_or_default();
            Project {
                id,
                slug,
                title: row.get("title"),
                summary: row.get("summary"),
                project_type: row.get("project_type"),
                download_count: row.get("download_count"),
                icon_url,
                owner: row.get("owner"),
                categories,
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            }
        })
        .collect();
    Ok((StatusCode::OK, Json(json!({ "projects": projects }))).into_response())
}
