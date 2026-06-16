use axum::response::{IntoResponse, Response};
use axum::{Json, extract::State, http::StatusCode};
use serde::Deserialize;
use serde_json::json;
use sqlx::Row;

use crate::error::AppError;
use crate::extract::AuthUser;

const PROJECT_TYPES: [&str; 4] = ["addon", "world", "resource_pack", "skin_pack"];
const VISIBILITIES: [&str; 3] = ["public", "unlisted", "private"];

fn default_visibility() -> String {
    "public".to_string()
}

#[derive(Deserialize)]
pub struct CreateRequest {
    title: String,
    project_type: String,
    #[serde(default)]
    summary: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    category_ids: Vec<String>,
    #[serde(default = "default_visibility")]
    visibility: String,
}

pub async fn create(
    State(pool): State<sqlx::PgPool>,
    AuthUser(user): AuthUser,
    Json(body): Json<CreateRequest>,
) -> Result<Response, AppError> {
    let owner_id = user.id;

    let title = body.title.trim();
    if title.is_empty() {
        return Err(AppError::bad_request("a title is required"));
    }
    if !PROJECT_TYPES.contains(&body.project_type.as_str()) {
        return Err(AppError::bad_request("invalid project type"));
    }
    if !VISIBILITIES.contains(&body.visibility.as_str()) {
        return Err(AppError::bad_request("invalid visibility"));
    }

    if !body.category_ids.is_empty() {
        let row = sqlx::query(
            "select count(*) as count from categories \
             where id = any($1::uuid[]) and project_type = $2",
        )
        .bind(&body.category_ids)
        .bind(&body.project_type)
        .fetch_one(&pool)
        .await
        .map_err(|_| AppError::bad_request("invalid category"))?;

        let count: i64 = row.get("count");
        if count as usize != body.category_ids.len() {
            return Err(AppError::bad_request("invalid category"));
        }
    }

    let slug = unique_slug(&pool, title).await?;

    let row = sqlx::query(
        r#"
        with new_project as (
            insert into projects (slug, title, summary, description, project_type, owner_id, visibility)
            values ($1, $2, $3, $4, $5, $6::uuid, $7)
            returning id, slug
        ), new_member as (
            insert into project_members (project_id, user_id, role)
            select id, $6::uuid, 'owner' from new_project
        ), new_categories as (
            insert into project_categories (project_id, category_id)
            select np.id, c.id from new_project np, unnest($8::uuid[]) as c(id)
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
    .bind(&body.visibility)
    .bind(&body.category_ids)
    .fetch_one(&pool)
    .await?;

    let id: String = row.get("id");
    let slug: String = row.get("slug");
    Ok((StatusCode::CREATED, Json(json!({ "id": id, "slug": slug }))).into_response())
}

pub(super) fn slugify(title: &str) -> String {
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
        if slug.is_empty() {
            "project".to_string()
        } else {
            slug
        }
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
