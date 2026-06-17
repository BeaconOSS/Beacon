use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router, extract::Path, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::Row;

use crate::constants;
use crate::error::AppError;
use crate::extract::ModeratorUser;
use crate::pack::{FileEntry, PackDiff, diff_indexes};
use crate::routes::sql::created_at_utc;
use crate::state::AppState;

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
    let rows = sqlx::query(concat!(
        r#"
        select
            p.id::text as id,
            p.slug,
            p.title,
            p.summary,
            p.project_type,
            p.icon_key,
            u.username as owner,
            "#,
        created_at_utc!("p.submitted_at", "submitted_at"),
        r#"
        from projects p
        join users u on u.id = p.owner_id
        where p.status = 'in_review'
        order by p.submitted_at asc nulls last, p.created_at asc
        "#,
    ))
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
struct GalleryItem {
    id: String,
    caption: String,
    url: String,
}

#[derive(Serialize)]
struct VersionFile {
    filename: String,
    size: i64,
    sha256: String,
}

#[derive(Serialize)]
struct VersionItem {
    version_number: String,
    name: String,
    channel: String,
    changelog: String,
    created_at: Option<String>,
    file: Option<VersionFile>,
}

#[derive(Serialize)]
struct AnalysisReport {
    status: String,
    error: String,
    mctools_version: String,
    analyzed_at: Option<String>,
    report: Option<serde_json::Value>,
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
    gallery: Vec<GalleryItem>,
    versions: Vec<VersionItem>,
    analysis: Option<AnalysisReport>,
    pack_diff: Option<PackDiff>,
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
    let row = sqlx::query(concat!(
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
            "#,
        created_at_utc!("p.created_at"),
        r#",
            "#,
        created_at_utc!("p.submitted_at", "submitted_at"),
        r#",
            (select count(*) from project_hearts h where h.project_id = p.id) as heart_count,
            (select count(*) from versions v where v.project_id = p.id) as version_count,
            (select count(*) from gallery_images g where g.project_id = p.id) as gallery_count
        from projects p
        where p.slug = $1
        "#,
    ))
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

    let owner_row = sqlx::query(concat!(
        r#"
        select
            u.username,
            "#,
        created_at_utc!("u.created_at", "member_since"),
        r#",
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
    ))
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

    let history_rows = sqlx::query(concat!(
        r#"
        select
            r.action,
            u.username as reviewer,
            r.notes,
            "#,
        created_at_utc!("r.created_at"),
        r#"
        from project_reviews r
        join users u on u.id = r.reviewer_id
        where r.project_id = $1::uuid
        order by r.created_at desc
        "#,
    ))
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

    let gallery_rows = sqlx::query(
        r#"
        select id::text as id, caption
        from gallery_images
        where project_id = $1::uuid
        order by position, created_at
        "#,
    )
    .bind(&id)
    .fetch_all(&pool)
    .await?;

    let gallery: Vec<GalleryItem> = gallery_rows
        .into_iter()
        .map(|row| {
            let image_id: String = row.get("id");
            GalleryItem {
                url: format!("/projects/{slug}/gallery/{image_id}"),
                caption: row.get("caption"),
                id: image_id,
            }
        })
        .collect();

    let version_rows = sqlx::query(concat!(
        r#"
        select
            v.version_number,
            v.name,
            v.channel,
            v.changelog,
            "#,
        created_at_utc!("v.created_at"),
        r#",
            f.filename,
            f.size,
            f.sha256
        from versions v
        left join files f on f.version_id = v.id and f.is_primary = true
        where v.project_id = $1::uuid
        order by v.created_at desc
        "#,
    ))
    .bind(&id)
    .fetch_all(&pool)
    .await?;

    let versions: Vec<VersionItem> = version_rows
        .into_iter()
        .map(|row| {
            let filename: Option<String> = row.get("filename");
            let file = filename.map(|filename| VersionFile {
                filename,
                size: row.get("size"),
                sha256: row.get("sha256"),
            });
            VersionItem {
                version_number: row.get("version_number"),
                name: row.get("name"),
                channel: row.get("channel"),
                changelog: row.get("changelog"),
                created_at: row.get("created_at"),
                file,
            }
        })
        .collect();

    let analysis_row = sqlx::query(concat!(
        r#"
        select
            a.report::text as report,
            a.error,
            a.mctools_version,
            "#,
        created_at_utc!("a.analyzed_at", "analyzed_at"),
        r#"
        from versions v
        join files f on f.version_id = v.id and f.is_primary = true
        left join version_file_analysis a on a.file_id = f.id
        where v.project_id = $1::uuid
        order by v.created_at desc
        limit 1
        "#,
    ))
    .bind(&id)
    .fetch_optional(&pool)
    .await?;

    let analysis = analysis_row.map(|row| {
        let report_text: Option<String> = row.get("report");
        let error: Option<String> = row.get("error");
        let mctools_version: Option<String> = row.get("mctools_version");
        let report = report_text.and_then(|text| serde_json::from_str(&text).ok());
        let error = error.unwrap_or_default();
        let status = if report.is_some() {
            "ready"
        } else if !error.is_empty() {
            "error"
        } else {
            "pending"
        };
        AnalysisReport {
            status: status.to_string(),
            error,
            mctools_version: mctools_version.unwrap_or_default(),
            analyzed_at: row.get("analyzed_at"),
            report,
        }
    });

    let index_rows = sqlx::query(
        r#"
        select a.file_index::text as file_index
        from versions v
        join files f on f.version_id = v.id and f.is_primary = true
        left join version_file_analysis a on a.file_id = f.id
        where v.project_id = $1::uuid
        order by v.created_at desc
        limit 2
        "#,
    )
    .bind(&id)
    .fetch_all(&pool)
    .await?;

    let parse_index = |row: &sqlx::postgres::PgRow| -> Option<Vec<FileEntry>> {
        let text: Option<String> = row.get("file_index");
        text.and_then(|text| serde_json::from_str(&text).ok())
    };

    let pack_diff = match index_rows.first().and_then(parse_index) {
        Some(new_index) => {
            let old_index = index_rows.get(1).and_then(parse_index).unwrap_or_default();
            Some(diff_indexes(&old_index, &new_index))
        }
        None => None,
    };

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
        gallery,
        versions,
        analysis,
        pack_diff,
    };

    Ok((StatusCode::OK, Json(result)).into_response())
}

#[derive(Serialize)]
struct ModeratorNote {
    id: String,
    author: String,
    body: String,
    created_at: Option<String>,
}

async fn project_id_for_slug(pool: &sqlx::PgPool, slug: &str) -> Result<String, AppError> {
    let row = sqlx::query("select id::text as id from projects where slug = $1")
        .bind(slug)
        .fetch_optional(pool)
        .await?;
    match row {
        Some(row) => Ok(row.get("id")),
        None => Err(AppError::not_found("project not found")),
    }
}

async fn list_moderator_notes(
    State(pool): State<sqlx::PgPool>,
    ModeratorUser(_): ModeratorUser,
    Path(slug): Path<String>,
) -> Result<Response, AppError> {
    let project_id = project_id_for_slug(&pool, &slug).await?;

    let rows = sqlx::query(concat!(
        r#"
        select
            n.id::text as id,
            u.username as author,
            n.body,
            "#,
        created_at_utc!("n.created_at"),
        r#"
        from project_moderator_notes n
        join users u on u.id = n.author_id
        where n.project_id = $1::uuid
        order by n.created_at desc
        "#,
    ))
    .bind(&project_id)
    .fetch_all(&pool)
    .await?;

    let notes: Vec<ModeratorNote> = rows
        .into_iter()
        .map(|row| ModeratorNote {
            id: row.get("id"),
            author: row.get("author"),
            body: row.get("body"),
            created_at: row.get("created_at"),
        })
        .collect();

    Ok((StatusCode::OK, Json(json!({ "notes": notes }))).into_response())
}

#[derive(Deserialize)]
struct AddNoteRequest {
    #[serde(default)]
    body: String,
}

async fn add_moderator_note(
    State(pool): State<sqlx::PgPool>,
    ModeratorUser(moderator): ModeratorUser,
    Path(slug): Path<String>,
    Json(payload): Json<AddNoteRequest>,
) -> Result<Response, AppError> {
    let body = payload.body.trim();
    if body.is_empty() {
        return Err(AppError::bad_request("note body cannot be empty"));
    }

    let project_id = project_id_for_slug(&pool, &slug).await?;

    let row = sqlx::query(concat!(
        r#"
        with inserted as (
            insert into project_moderator_notes (project_id, author_id, body)
            values ($1::uuid, $2::uuid, $3)
            returning id, author_id, body, created_at
        )
        select
            inserted.id::text as id,
            u.username as author,
            inserted.body,
            "#,
        created_at_utc!("inserted.created_at"),
        r#"
        from inserted
        join users u on u.id = inserted.author_id
        "#,
    ))
    .bind(&project_id)
    .bind(&moderator.id)
    .bind(body)
    .fetch_one(&pool)
    .await?;

    let note = ModeratorNote {
        id: row.get("id"),
        author: row.get("author"),
        body: row.get("body"),
        created_at: row.get("created_at"),
    };

    Ok((StatusCode::CREATED, Json(note)).into_response())
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/moderation/projects", get(queue))
        .route("/projects/{slug}/review", post(review))
        .route("/projects/{slug}/pending", get(pending_review))
        .route(
            "/projects/{slug}/moderator-notes",
            get(list_moderator_notes).post(add_moderator_note),
        )
}
