use axum::response::{IntoResponse, Response};
use axum::{Json, extract::Path, extract::State, http::StatusCode};
use serde::Deserialize;
use serde_json::json;
use sqlx::Row;

use crate::constants;
use crate::error::AppError;
use crate::extract::ModeratorUser;

#[derive(Deserialize)]
pub struct ReviewRequest {
    action: String,
    #[serde(default)]
    notes: String,
}

pub async fn review(
    State(pool): State<sqlx::PgPool>,
    ModeratorUser(moderator): ModeratorUser,
    Path(slug): Path<String>,
    Json(body): Json<ReviewRequest>,
) -> Result<Response, AppError> {
    if !constants::REVIEW_ACTIONS.contains(&body.action.as_str()) {
        return Err(AppError::bad_request("invalid review action"));
    }

    let project = sqlx::query("select id::text as id, status from projects where slug = $1")
        .bind(&slug)
        .fetch_optional(&pool)
        .await?;

    let Some(project) = project else {
        return Err(AppError::not_found("project not found"));
    };
    let project_id: String = project.get("id");
    let status: String = project.get("status");

    if status != constants::STATUS_IN_REVIEW {
        return Err(AppError::conflict(
            "this project is not currently awaiting review",
        ));
    }

    let new_status = match body.action.as_str() {
        constants::REVIEW_ACTION_APPROVE => constants::STATUS_APPROVED,
        constants::REVIEW_ACTION_REJECT => constants::STATUS_REJECTED,
        _ => constants::STATUS_CHANGES_REQUESTED,
    };

    sqlx::query("update projects set status = $1, updated_at = now() where id = $2::uuid")
        .bind(new_status)
        .bind(&project_id)
        .execute(&pool)
        .await?;

    if body.action == constants::REVIEW_ACTION_APPROVE {
        sqlx::query(
            "update projects set \
                 published_title = title, \
                 published_summary = summary, \
                 published_description = description, \
                 published_icon_key = icon_key, \
                 published_license = license, \
                 published_at = now(), \
                 pending_changelog = '' \
             where id = $1::uuid",
        )
        .bind(&project_id)
        .execute(&pool)
        .await?;

        sqlx::query("delete from project_published_categories where project_id = $1::uuid")
            .bind(&project_id)
            .execute(&pool)
            .await?;
        sqlx::query(
            "insert into project_published_categories (project_id, category_id) \
             select project_id, category_id from project_categories where project_id = $1::uuid",
        )
        .bind(&project_id)
        .execute(&pool)
        .await?;
    }

    sqlx::query(
        "insert into project_reviews (project_id, reviewer_id, action, notes) \
         values ($1::uuid, $2::uuid, $3, $4)",
    )
    .bind(&project_id)
    .bind(&moderator.id)
    .bind(&body.action)
    .bind(body.notes.trim())
    .execute(&pool)
    .await?;

    Ok((StatusCode::OK, Json(json!({ "status": new_status }))).into_response())
}
