use axum::response::{IntoResponse, Response};
use axum::{Json, extract::Path, extract::State, http::StatusCode};
use serde::Deserialize;
use serde_json::json;

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
    visibility: Option<String>,
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

    let mut new_slug = slug.clone();

    if let Some(title) = body.title.as_ref() {
        let title = title.trim();
        if title.is_empty() {
            return Err(AppError::bad_request("a title is required"));
        }
        sqlx::query("update projects set title = $1 where id = $2::uuid")
            .bind(title)
            .bind(&project_id)
            .execute(&pool)
            .await?;
    }

    if let Some(summary) = body.summary.as_ref() {
        sqlx::query("update projects set summary = $1 where id = $2::uuid")
            .bind(summary.trim())
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
        }
    }

    sqlx::query("update projects set updated_at = now() where id = $1::uuid")
        .bind(&project_id)
        .execute(&pool)
        .await?;

    Ok((StatusCode::OK, Json(json!({ "slug": new_slug }))).into_response())
}
