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
}

pub async fn update(
    State(pool): State<sqlx::PgPool>,
    AuthUser(user): AuthUser,
    Path(slug): Path<String>,
    Json(body): Json<UpdateRequest>,
) -> Result<Response, AppError> {
    let project_id = require_project_owner(&pool, &slug, &user.id).await?;

    let current =
        sqlx::query("select status, title, summary, description from projects where id = $1::uuid")
            .bind(&project_id)
            .fetch_one(&pool)
            .await?;
    let current_status: String = current.get("status");
    let current_title: String = current.get("title");
    let current_summary: String = current.get("summary");
    let current_description: String = current.get("description");

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
