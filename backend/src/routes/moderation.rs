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
struct OwnerContext {
    username: String,
    member_since: Option<String>,
    project_count: i64,
    approved_count: i64,
    rejected_count: i64,
}

#[derive(Serialize)]
struct ProjectLinks {
    website_url: String,
    source_url: String,
    issues_url: String,
    wiki_url: String,
    discord_url: String,
}

#[derive(Serialize)]
struct ProjectFacts {
    visibility: String,
    monetization_enabled: bool,
    creator_share: i32,
    heart_count: i64,
    download_count: i64,
    version_count: i64,
    gallery_count: i64,
    created_at: Option<String>,
}

#[derive(Serialize)]
struct ReviewHistoryEntry {
    action: String,
    reviewer: String,
    notes: String,
    created_at: Option<String>,
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
    owner: OwnerContext,
    links: ProjectLinks,
    facts: ProjectFacts,
    history: Vec<ReviewHistoryEntry>,
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
            p.visibility,
            p.monetization_enabled,
            p.creator_share,
            p.download_count,
            p.website_url,
            p.source_url,
            p.issues_url,
            p.wiki_url,
            p.discord_url,
            p.owner_id::text as owner_id,
            p.published_title,
            p.published_summary,
            p.published_description,
            p.published_license,
            p.published_icon_key,
            p.published_at is not null as is_published,
            p.pending_changelog,
            to_char(p.created_at at time zone 'utc', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') as created_at,
            to_char(p.submitted_at at time zone 'utc', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') as submitted_at,
            (select count(*) from project_hearts h where h.project_id = p.id) as heart_count,
            (select count(*) from versions v where v.project_id = p.id) as version_count,
            (select count(*) from gallery_images g where g.project_id = p.id) as gallery_count
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
    let owner_id: String = row.get("owner_id");
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

    let links = ProjectLinks {
        website_url: row.get("website_url"),
        source_url: row.get("source_url"),
        issues_url: row.get("issues_url"),
        wiki_url: row.get("wiki_url"),
        discord_url: row.get("discord_url"),
    };

    let facts = ProjectFacts {
        visibility: row.get("visibility"),
        monetization_enabled: row.get("monetization_enabled"),
        creator_share: row.get("creator_share"),
        heart_count: row.get("heart_count"),
        download_count: row.get("download_count"),
        version_count: row.get("version_count"),
        gallery_count: row.get("gallery_count"),
        created_at: row.get("created_at"),
    };

    let owner_row = sqlx::query(
        r#"
        select
            u.username,
            to_char(u.created_at at time zone 'utc', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') as member_since,
            (select count(*) from projects p where p.owner_id = u.id) as project_count,
            (select count(*) from project_reviews r
                join projects p on p.id = r.project_id
                where p.owner_id = u.id and r.action = 'approve') as approved_count,
            (select count(*) from project_reviews r
                join projects p on p.id = r.project_id
                where p.owner_id = u.id and r.action = 'reject') as rejected_count
        from users u
        where u.id = $1::uuid
        "#,
    )
    .bind(&owner_id)
    .fetch_one(&pool)
    .await?;

    let owner = OwnerContext {
        username: owner_row.get("username"),
        member_since: owner_row.get("member_since"),
        project_count: owner_row.get("project_count"),
        approved_count: owner_row.get("approved_count"),
        rejected_count: owner_row.get("rejected_count"),
    };

    let history_rows = sqlx::query(
        r#"
        select
            r.action,
            u.username as reviewer,
            r.notes,
            to_char(r.created_at at time zone 'utc', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') as created_at
        from project_reviews r
        join users u on u.id = r.reviewer_id
        where r.project_id = $1::uuid
        order by r.created_at desc
        "#,
    )
    .bind(&id)
    .fetch_all(&pool)
    .await?;

    let history: Vec<ReviewHistoryEntry> = history_rows
        .into_iter()
        .map(|row| ReviewHistoryEntry {
            action: row.get("action"),
            reviewer: row.get("reviewer"),
            notes: row.get("notes"),
            created_at: row.get("created_at"),
        })
        .collect();

    let result = PendingReview {
        status: row.get("status"),
        submitted_at: row.get("submitted_at"),
        changelog: row.get("pending_changelog"),
        is_first_review: !is_published,
        icon_changed,
        published,
        pending,
        owner,
        links,
        facts,
        history,
    };

    Ok((StatusCode::OK, Json(result)).into_response())
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/moderation/projects", get(queue))
        .route("/projects/{slug}/review", post(review))
        .route("/projects/{slug}/pending", get(pending_review))
}
