use axum::response::{IntoResponse, Response};
use axum::{Json, extract::Path, extract::State, http::StatusCode};
use serde::Deserialize;
use serde_json::json;
use sqlx::Row;

use crate::constants;
use crate::error::AppError;
use crate::extract::AuthUser;
use crate::routes::owner::{has_pending_changes, require_project_owner};

#[derive(Deserialize)]
pub struct SubmitRequest {
    #[serde(default)]
    changelog: String,
}

pub async fn submit(
    State(pool): State<sqlx::PgPool>,
    AuthUser(user): AuthUser,
    Path(slug): Path<String>,
    Json(body): Json<SubmitRequest>,
) -> Result<Response, AppError> {
    let project_id = require_project_owner(&pool, &slug, &user.id).await?;

    let row = sqlx::query(
        r#"
        select
            p.status,
            p.title,
            p.summary,
            p.description,
            (select count(*) from versions v where v.project_id = p.id) as version_count
        from projects p
        where p.id = $1::uuid
        "#,
    )
    .bind(&project_id)
    .fetch_one(&pool)
    .await?;

    let status: String = row.get("status");
    let resubmitting_published = status == constants::STATUS_APPROVED;
    if status != constants::STATUS_DRAFT
        && status != constants::STATUS_CHANGES_REQUESTED
        && status != constants::STATUS_REJECTED
        && status != constants::STATUS_APPROVED
    {
        return Err(AppError::conflict(
            "this project has already been submitted for review",
        ));
    }
    if resubmitting_published && !has_pending_changes(&pool, &project_id).await? {
        return Err(AppError::conflict(
            "there are no changes to submit for review",
        ));
    }

    let title: String = row.get("title");
    let summary: String = row.get("summary");
    let description: String = row.get("description");
    let version_count: i64 = row.get("version_count");

    let summary = summary.trim();
    let description = description.trim();

    if version_count < 1 {
        return Err(AppError::bad_request(
            "upload at least one version before submitting for review",
        ));
    }
    if description.is_empty() {
        return Err(AppError::bad_request(
            "add a description before submitting for review",
        ));
    }
    if summary.is_empty() || summary.eq_ignore_ascii_case(title.trim()) {
        return Err(AppError::bad_request(
            "add a summary that is different from the project name",
        ));
    }

    sqlx::query(
        "update projects set status = 'in_review', submitted_at = now(), updated_at = now(), \
             pending_changelog = $2 \
         where id = $1::uuid",
    )
    .bind(&project_id)
    .bind(body.changelog.trim())
    .execute(&pool)
    .await?;

    Ok((StatusCode::OK, Json(json!({ "status": "in_review" }))).into_response())
}

pub async fn withdraw(
    State(pool): State<sqlx::PgPool>,
    AuthUser(user): AuthUser,
    Path(slug): Path<String>,
) -> Result<Response, AppError> {
    let project_id = require_project_owner(&pool, &slug, &user.id).await?;

    let row = sqlx::query(
        "select status, published_at is not null as is_published \
         from projects where id = $1::uuid",
    )
    .bind(&project_id)
    .fetch_one(&pool)
    .await?;

    let status: String = row.get("status");
    if status != constants::STATUS_IN_REVIEW {
        return Err(AppError::conflict(
            "this project is not currently awaiting review",
        ));
    }

    let is_published: bool = row.get("is_published");
    let new_status = if is_published {
        constants::STATUS_APPROVED
    } else {
        constants::STATUS_DRAFT
    };

    sqlx::query("update projects set status = $1, updated_at = now() where id = $2::uuid")
        .bind(new_status)
        .bind(&project_id)
        .execute(&pool)
        .await?;

    Ok((StatusCode::OK, Json(json!({ "status": new_status }))).into_response())
}
