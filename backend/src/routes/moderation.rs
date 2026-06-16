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

    if body.action == "approve" {
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

#[derive(Serialize)]
struct RevisionContent {
    title: String,
    summary: String,
    description: String,
    license: String,
    icon_url: Option<String>,
    categories: Vec<String>,
}

#[derive(Serialize)]
struct PendingReview {
    status: String,
    submitted_at: Option<String>,
    changelog: String,
    is_first_review: bool,
    icon_changed: bool,
    published: Option<RevisionContent>,
    pending: RevisionContent,
}

async fn category_names(
    pool: &sqlx::PgPool,
    published: bool,
    project_id: &str,
) -> Result<Vec<String>, AppError> {
    let sql = if published {
        "select c.name from project_published_categories pc \
         join categories c on c.id = pc.category_id \
         where pc.project_id = $1::uuid order by c.ordering"
    } else {
        "select c.name from project_categories pc \
         join categories c on c.id = pc.category_id \
         where pc.project_id = $1::uuid order by c.ordering"
    };
    let rows = sqlx::query(sql).bind(project_id).fetch_all(pool).await?;
    Ok(rows.into_iter().map(|row| row.get("name")).collect())
}

async fn pending_review(
    State(pool): State<sqlx::PgPool>,
    ModeratorUser(_): ModeratorUser,
    Path(slug): Path<String>,
) -> Result<Response, AppError> {
    let row = sqlx::query(
        r#"
        select
            p.id::text as id,
            p.status,
            p.title,
            p.summary,
            p.description,
            p.license,
            p.icon_key,
            p.published_title,
            p.published_summary,
            p.published_description,
            p.published_license,
            p.published_icon_key,
            p.published_at is not null as is_published,
            p.pending_changelog,
            to_char(p.submitted_at at time zone 'utc', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') as submitted_at
        from projects p
        where p.slug = $1
        "#,
    )
    .bind(&slug)
    .fetch_optional(&pool)
    .await?;

    let Some(row) = row else {
        return Err(AppError::not_found("project not found"));
    };

    let id: String = row.get("id");
    let is_published: bool = row.get("is_published");
    let icon_key: Option<String> = row.get("icon_key");
    let published_icon_key: Option<String> = row.get("published_icon_key");
    let icon_changed = icon_key != published_icon_key;

    let pending_categories = category_names(&pool, false, &id).await?;

    let pending = RevisionContent {
        title: row.get("title"),
        summary: row.get("summary"),
        description: row.get("description"),
        license: row.get("license"),
        icon_url: icon_key
            .as_ref()
            .map(|_| format!("/projects/{slug}/icon?revision=pending")),
        categories: pending_categories,
    };

    let published = if is_published {
        let published_categories = category_names(&pool, true, &id).await?;
        Some(RevisionContent {
            title: row.get("published_title"),
            summary: row.get("published_summary"),
            description: row.get("published_description"),
            license: row.get("published_license"),
            icon_url: published_icon_key
                .as_ref()
                .map(|_| format!("/projects/{slug}/icon")),
            categories: published_categories,
        })
    } else {
        None
    };

    let result = PendingReview {
        status: row.get("status"),
        submitted_at: row.get("submitted_at"),
        changelog: row.get("pending_changelog"),
        is_first_review: !is_published,
        icon_changed,
        published,
        pending,
    };

    Ok((StatusCode::OK, Json(result)).into_response())
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/moderation/projects", get(queue))
        .route("/projects/{slug}/review", post(review))
        .route("/projects/{slug}/pending", get(pending_review))
}
