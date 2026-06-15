use axum::response::{IntoResponse, Response};
use axum::{Json, extract::Path, extract::State, http::StatusCode};
use axum_extra::extract::cookie::CookieJar;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::Row;

use crate::error::error;
use crate::session;

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

const PROJECT_TYPES: [&str; 4] = ["addon", "world", "resource_pack", "skin_pack"];

#[derive(Deserialize)]
pub struct CreateRequest {
    title: String,
    project_type: String,
    #[serde(default)]
    summary: String,
    #[serde(default)]
    description: String,
}

pub async fn create(
    State(pool): State<sqlx::PgPool>,
    jar: CookieJar,
    Json(body): Json<CreateRequest>,
) -> Response {
    let Some(token) = jar.get(session::SESSION_COOKIE).map(|c| c.value().to_string()) else {
        return error(StatusCode::UNAUTHORIZED, "not signed in").into_response();
    };

    let owner_id = match session::lookup(&pool, &token).await {
        Ok(Some(user)) => user.id,
        Ok(None) => return error(StatusCode::UNAUTHORIZED, "not signed in").into_response(),
        Err(_) => {
            return error(StatusCode::INTERNAL_SERVER_ERROR, "could not read session")
                .into_response();
        }
    };

    let title = body.title.trim();
    if title.is_empty() {
        return error(StatusCode::BAD_REQUEST, "a title is required").into_response();
    }
    if !PROJECT_TYPES.contains(&body.project_type.as_str()) {
        return error(StatusCode::BAD_REQUEST, "invalid project type").into_response();
    }

    let slug = match unique_slug(&pool, title).await {
        Ok(slug) => slug,
        Err(_) => {
            return error(StatusCode::INTERNAL_SERVER_ERROR, "could not create project")
                .into_response();
        }
    };

    let row = sqlx::query(
        r#"
        with new_project as (
            insert into projects (slug, title, summary, description, project_type, owner_id, published)
            values ($1, $2, $3, $4, $5, $6::uuid, true)
            returning id, slug
        ), new_member as (
            insert into project_members (project_id, user_id, role)
            select id, $6::uuid, 'owner' from new_project
        )
        select id::text as id, slug from new_project
        "#,
    )
    .bind(&slug)
    .bind(title)
    .bind(body.summary.trim())
    .bind(body.description.trim())
    .bind(&body.project_type)
    .bind(&owner_id)
    .fetch_one(&pool)
    .await;

    match row {
        Ok(row) => {
            let id: String = row.get("id");
            let slug: String = row.get("slug");
            (StatusCode::CREATED, Json(json!({ "id": id, "slug": slug }))).into_response()
        }
        Err(_) => {
            error(StatusCode::INTERNAL_SERVER_ERROR, "could not create project").into_response()
        }
    }
}

fn slugify(title: &str) -> String {
    let mut slug = String::new();
    let mut last_dash = false;
    for ch in title.chars() {
        if ch.is_ascii_alphanumeric() {
            slug.push(ch.to_ascii_lowercase());
            last_dash = false;
        } else if !last_dash && !slug.is_empty() {
            slug.push('-');
            last_dash = true;
        }
    }
    slug.trim_end_matches('-').to_string()
}

async fn unique_slug(pool: &sqlx::PgPool, title: &str) -> Result<String, sqlx::Error> {
    let base = {
        let slug = slugify(title);
        if slug.is_empty() { "project".to_string() } else { slug }
    };
    let mut candidate = base.clone();
    let mut suffix = 1;

    loop {
        let taken = sqlx::query("select 1 from projects where slug = $1")
            .bind(&candidate)
            .fetch_optional(pool)
            .await?
            .is_some();

        if !taken {
            return Ok(candidate);
        }

        suffix += 1;
        candidate = format!("{base}-{suffix}");
    }
}
