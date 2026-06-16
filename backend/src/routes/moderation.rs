use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router, extract::Path, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::Row;

use crate::error::AppError;
use crate::extract::ModeratorUser;
use crate::state::AppState;

const REVIEW_ACTIONS: [&str; 3] = ["approve", "reject", "request_changes"];

#[derive(Serialize)]
struct QueueItem {
    id: String,
    slug: String,
    title: String,
    summary: String,
    project_type: String,
    owner: String,
    icon_url: Option<String>,
    submitted_at: Option<String>,
}

async fn queue(
    State(pool): State<sqlx::PgPool>,
    ModeratorUser(_): ModeratorUser,
) -> Result<Response, AppError> {
    let rows = sqlx::query(
        r#"
        select
            p.id::text as id,
            p.slug,
            p.title,
            p.summary,
            p.project_type,
            p.icon_key,
            u.username as owner,
            to_char(p.submitted_at at time zone 'utc', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') as submitted_at
        from projects p
        join users u on u.id = p.owner_id
        where p.status = 'in_review'
        order by p.submitted_at asc nulls last, p.created_at asc
        "#,
    )
    .fetch_all(&pool)
    .await?;

    let items: Vec<QueueItem> = rows
        .into_iter()
        .map(|row| {
            let slug: String = row.get("slug");
            let icon_key: Option<String> = row.get("icon_key");
            QueueItem {
                icon_url: icon_key.map(|_| format!("/projects/{slug}/icon")),
                id: row.get("id"),
                title: row.get("title"),
                summary: row.get("summary"),
                project_type: row.get("project_type"),
                owner: row.get("owner"),
                submitted_at: row.get("submitted_at"),
                slug,
            }
        })
        .collect();

    Ok((StatusCode::OK, Json(json!({ "projects": items }))).into_response())
}

#[derive(Deserialize)]
struct ReviewRequest {
    action: String,
    #[serde(default)]
    notes: String,
}

async fn review(
    State(pool): State<sqlx::PgPool>,
    ModeratorUser(moderator): ModeratorUser,
    Path(slug): Path<String>,
    Json(body): Json<ReviewRequest>,
) -> Result<Response, AppError> {
    if !REVIEW_ACTIONS.contains(&body.action.as_str()) {
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

    if status != "in_review" {
        return Err(AppError::conflict(
            "this project is not currently awaiting review",
        ));
    }

    let new_status = match body.action.as_str() {
        "approve" => "approved",
        "reject" => "rejected",
        _ => "changes_requested",
    };

    sqlx::query("update projects set status = $1, updated_at = now() where id = $2::uuid")
        .bind(new_status)
        .bind(&project_id)
        .execute(&pool)
        .await?;

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

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/moderation/projects", get(queue))
        .route("/projects/{slug}/review", post(review))
}
