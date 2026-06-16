use axum::response::{IntoResponse, Response};
use axum::{Json, extract::Path, extract::State, http::StatusCode};
use axum_extra::extract::cookie::CookieJar;
use serde::Serialize;
use sqlx::Row;

use crate::error::AppError;
use crate::routes::access::project_for_viewer;

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
    visibility: String,
    status: String,
    download_count: i64,
    owner: String,
    icon_url: Option<String>,
    categories: Vec<CategoryTag>,
    created_at: String,
}

pub async fn detail(
    State(pool): State<sqlx::PgPool>,
    jar: CookieJar,
    Path(slug): Path<String>,
) -> Result<Response, AppError> {
    project_for_viewer(&pool, &jar, &slug).await?;

    let row = sqlx::query(concat!(
        r#"
        select
            p.id::text as id,
            p.slug,
            p.title,
            p.summary,
            p.description,
            p.project_type,
            p.visibility,
            p.status,
            p.download_count,
            p.icon_key,
            u.username as owner,
            "#,
        crate::routes::sql::created_at_utc!("p.created_at"),
        r#"
        from projects p
        join users u on u.id = p.owner_id
        where p.slug = $1
        "#,
    ))
    .bind(&slug)
    .fetch_optional(&pool)
    .await?;

    let Some(row) = row else {
        return Err(AppError::not_found("project not found"));
    };

    let id: String = row.get("id");

    let icon_key: Option<String> = row.get("icon_key");
    let icon_url = icon_key.map(|_| format!("/projects/{slug}/icon"));

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
    .await?;

    let categories = category_rows
        .into_iter()
        .map(|row| CategoryTag {
            slug: row.get("slug"),
            name: row.get("name"),
        })
        .collect();

    let project = ProjectDetail {
        id,
        slug: row.get("slug"),
        title: row.get("title"),
        summary: row.get("summary"),
        description: row.get("description"),
        project_type: row.get("project_type"),
        visibility: row.get("visibility"),
        status: row.get("status"),
        download_count: row.get("download_count"),
        owner: row.get("owner"),
        icon_url,
        categories,
        created_at: row.get("created_at"),
    };
    Ok((StatusCode::OK, Json(project)).into_response())
}
