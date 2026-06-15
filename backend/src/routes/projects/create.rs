use axum::response::{IntoResponse, Response};
use axum::{Json, extract::State, http::StatusCode};
use serde::Deserialize;
use serde_json::json;
use sqlx::Row;

use crate::error::error;
use crate::extract::AuthUser;

const PROJECT_TYPES: [&str; 4] = ["addon", "world", "resource_pack", "skin_pack"];

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
}

pub async fn create(
    State(pool): State<sqlx::PgPool>,
    AuthUser(user): AuthUser,
    Json(body): Json<CreateRequest>,
) -> Response {
    let owner_id = user.id;

    let title = body.title.trim();
    if title.is_empty() {
        return error(StatusCode::BAD_REQUEST, "a title is required").into_response();
    }
    if !PROJECT_TYPES.contains(&body.project_type.as_str()) {
        return error(StatusCode::BAD_REQUEST, "invalid project type").into_response();
    }

    if !body.category_ids.is_empty() {
        let valid = sqlx::query(
            "select count(*) as count from categories \
             where id = any($1::uuid[]) and project_type = $2",
        )
        .bind(&body.category_ids)
        .bind(&body.project_type)
        .fetch_one(&pool)
        .await;

        match valid {
            Ok(row) => {
                let count: i64 = row.get("count");
                if count as usize != body.category_ids.len() {
                    return error(StatusCode::BAD_REQUEST, "invalid category").into_response();
                }
            }
            Err(_) => {
                return error(StatusCode::BAD_REQUEST, "invalid category").into_response();
            }
        }
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
        ), new_categories as (
            insert into project_categories (project_id, category_id)
            select np.id, c.id from new_project np, unnest($7::uuid[]) as c(id)
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
    .bind(&body.category_ids)
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
