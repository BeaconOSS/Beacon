use axum::response::{IntoResponse, Response};
use axum::{Json, extract::Path, extract::State, http::StatusCode};
use serde_json::json;
use sqlx::Row;

use crate::error::AppError;
use crate::extract::AuthUser;
use crate::routes::owner::require_project_owner;

pub async fn submit(
    State(pool): State<sqlx::PgPool>,
    AuthUser(user): AuthUser,
    Path(slug): Path<String>,
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
    if status != "draft" && status != "changes_requested" && status != "rejected" {
        return Err(AppError::conflict(
            "this project has already been submitted for review",
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
        "update projects set status = 'in_review', submitted_at = now(), updated_at = now() \
         where id = $1::uuid",
    )
    .bind(&project_id)
    .execute(&pool)
    .await?;

    Ok((StatusCode::OK, Json(json!({ "status": "in_review" }))).into_response())
}
