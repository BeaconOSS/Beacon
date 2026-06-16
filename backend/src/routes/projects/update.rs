use axum::response::{IntoResponse, Response};
use axum::{Json, extract::Path, extract::State, http::StatusCode};
use serde::Deserialize;
use serde_json::json;
use sqlx::Row;

use crate::error::AppError;
use crate::extract::AuthUser;
use crate::routes::owner::require_project_owner;

use super::create::slugify;

const VISIBILITIES: [&str; 3] = ["public", "unlisted", "private"];

#[derive(Deserialize)]
pub struct UpdateRequest {
    title: Option<String>,
    slug: Option<String>,
    summary: Option<String>,
    description: Option<String>,
    visibility: Option<String>,
    license: Option<String>,
    monetization_enabled: Option<bool>,
    creator_share: Option<i32>,
    category_ids: Option<Vec<String>>,
    website_url: Option<String>,
    source_url: Option<String>,
    issues_url: Option<String>,
    wiki_url: Option<String>,
    discord_url: Option<String>,
    changelog: Option<String>,
}

fn validate_url(value: &str) -> Result<(), AppError> {
    if value.is_empty() {
        return Ok(());
    }
    if value.starts_with("http://") || value.starts_with("https://") {
        Ok(())
    } else {
        Err(AppError::bad_request(
            "links must start with http:// or https://",
        ))
    }
}

pub async fn update(
    State(pool): State<sqlx::PgPool>,
    AuthUser(user): AuthUser,
    Path(slug): Path<String>,
    Json(body): Json<UpdateRequest>,
) -> Result<Response, AppError> {
    let project_id = require_project_owner(&pool, &slug, &user.id).await?;

    let current = sqlx::query(
        "select status, title, summary, description, project_type from projects where id = $1::uuid",
    )
    .bind(&project_id)
    .fetch_one(&pool)
    .await?;
    let current_status: String = current.get("status");
    let current_title: String = current.get("title");
    let current_summary: String = current.get("summary");
    let current_description: String = current.get("description");
    let current_project_type: String = current.get("project_type");

    if current_status == "in_review" {
        let editing_content = body.title.is_some()
            || body.slug.is_some()
            || body.summary.is_some()
            || body.description.is_some()
            || body.visibility.is_some()
            || body.license.is_some()
            || body.monetization_enabled.is_some()
            || body.creator_share.is_some()
            || body.category_ids.is_some()
            || body.website_url.is_some()
            || body.source_url.is_some()
            || body.issues_url.is_some()
            || body.wiki_url.is_some()
            || body.discord_url.is_some();
        if editing_content {
            return Err(AppError::conflict(
                "this project is locked while it is under review",
            ));
        }
    }

    let mut new_slug = slug.clone();
    let mut sensitive_changed = false;

    if let Some(title) = body.title.as_ref() {
        let title = title.trim();
        if title.is_empty() {
            return Err(AppError::bad_request("a title is required"));
        }
        if title != current_title {
            sensitive_changed = true;
        }
        sqlx::query("update projects set title = $1 where id = $2::uuid")
            .bind(title)
            .bind(&project_id)
            .execute(&pool)
            .await?;
    }

    if let Some(summary) = body.summary.as_ref() {
        let summary = summary.trim();
        if summary != current_summary {
            sensitive_changed = true;
        }
        sqlx::query("update projects set summary = $1 where id = $2::uuid")
            .bind(summary)
            .bind(&project_id)
            .execute(&pool)
            .await?;
    }

    if let Some(description) = body.description.as_ref() {
        let description = description.trim();
        if description != current_description {
            sensitive_changed = true;
        }
        sqlx::query("update projects set description = $1 where id = $2::uuid")
            .bind(description)
            .bind(&project_id)
            .execute(&pool)
            .await?;
    }

    if let Some(visibility) = body.visibility.as_ref() {
        if !VISIBILITIES.contains(&visibility.as_str()) {
            return Err(AppError::bad_request("invalid visibility"));
        }
        sqlx::query("update projects set visibility = $1 where id = $2::uuid")
            .bind(visibility)
            .bind(&project_id)
            .execute(&pool)
            .await?;
    }

    if let Some(license) = body.license.as_ref() {
        sqlx::query("update projects set license = $1 where id = $2::uuid")
            .bind(license.trim())
            .bind(&project_id)
            .execute(&pool)
            .await?;
    }

    if let Some(enabled) = body.monetization_enabled {
        sqlx::query("update projects set monetization_enabled = $1 where id = $2::uuid")
            .bind(enabled)
            .bind(&project_id)
            .execute(&pool)
            .await?;
    }

    if let Some(share) = body.creator_share {
        if !(0..=80).contains(&share) {
            return Err(AppError::bad_request(
                "creator share must be between 0 and 80",
            ));
        }
        sqlx::query("update projects set creator_share = $1 where id = $2::uuid")
            .bind(share)
            .bind(&project_id)
            .execute(&pool)
            .await?;
    }

    let link_updates: [(&str, Option<&String>); 5] = [
        ("website_url", body.website_url.as_ref()),
        ("source_url", body.source_url.as_ref()),
        ("issues_url", body.issues_url.as_ref()),
        ("wiki_url", body.wiki_url.as_ref()),
        ("discord_url", body.discord_url.as_ref()),
    ];
    for (column, value) in link_updates {
        if let Some(value) = value {
            let value = value.trim();
            validate_url(value)?;
            let statement = match column {
                "website_url" => "update projects set website_url = $1 where id = $2::uuid",
                "source_url" => "update projects set source_url = $1 where id = $2::uuid",
                "issues_url" => "update projects set issues_url = $1 where id = $2::uuid",
                "wiki_url" => "update projects set wiki_url = $1 where id = $2::uuid",
                _ => "update projects set discord_url = $1 where id = $2::uuid",
            };
            sqlx::query(statement)
                .bind(value)
                .bind(&project_id)
                .execute(&pool)
                .await?;
        }
    }

    if let Some(category_ids) = body.category_ids.as_ref() {
        if !category_ids.is_empty() {
            let row = sqlx::query(
                "select count(*) as count from categories \
                 where id = any($1::uuid[]) and project_type = $2",
            )
            .bind(category_ids)
            .bind(&current_project_type)
            .fetch_one(&pool)
            .await
            .map_err(|_| AppError::bad_request("invalid category"))?;

            let count: i64 = row.get("count");
            if count as usize != category_ids.len() {
                return Err(AppError::bad_request("invalid category"));
            }
        }

        let existing_rows = sqlx::query(
            "select category_id::text as id from project_categories where project_id = $1::uuid",
        )
        .bind(&project_id)
        .fetch_all(&pool)
        .await?;
        let mut existing_ids: Vec<String> = existing_rows.iter().map(|r| r.get("id")).collect();
        existing_ids.sort();
        let mut new_ids = category_ids.clone();
        new_ids.sort();

        if existing_ids != new_ids {
            sensitive_changed = true;
            sqlx::query("delete from project_categories where project_id = $1::uuid")
                .bind(&project_id)
                .execute(&pool)
                .await?;
            sqlx::query(
                "insert into project_categories (project_id, category_id) \
                 select $1::uuid, c.id from unnest($2::uuid[]) as c(id)",
            )
            .bind(&project_id)
            .bind(category_ids)
            .execute(&pool)
            .await?;
        }
    }

    if let Some(requested) = body.slug.as_ref() {
        let normalized = slugify(requested);
        if normalized.is_empty() {
            return Err(AppError::bad_request("invalid url"));
        }
        if normalized != slug {
            let taken = sqlx::query("select 1 from projects where slug = $1 and id <> $2::uuid")
                .bind(&normalized)
                .bind(&project_id)
                .fetch_optional(&pool)
                .await?
                .is_some();
            if taken {
                return Err(AppError::conflict("that url is already taken"));
            }
            sqlx::query("update projects set slug = $1 where id = $2::uuid")
                .bind(&normalized)
                .bind(&project_id)
                .execute(&pool)
                .await?;
            new_slug = normalized;
            sensitive_changed = true;
        }
    }

    if let Some(changelog) = body.changelog.as_ref() {
        sqlx::query("update projects set pending_changelog = $1 where id = $2::uuid")
            .bind(changelog.trim())
            .bind(&project_id)
            .execute(&pool)
            .await?;
    }

    let new_status = if current_status == "approved" && sensitive_changed {
        sqlx::query(
            "update projects set status = 'in_review', submitted_at = now() where id = $1::uuid",
        )
        .bind(&project_id)
        .execute(&pool)
        .await?;
        "in_review".to_string()
    } else {
        current_status
    };

    sqlx::query("update projects set updated_at = now() where id = $1::uuid")
        .bind(&project_id)
        .execute(&pool)
        .await?;

    Ok((
        StatusCode::OK,
        Json(json!({ "slug": new_slug, "status": new_status })),
    )
        .into_response())
}
