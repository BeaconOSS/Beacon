use axum::response::{IntoResponse, Response};
use axum::{Json, extract::Path, extract::State, http::StatusCode};
use serde::Serialize;
use serde_json::json;
use sqlx::Row;

use crate::error::AppError;
use crate::extract::AuthUser;

#[derive(Serialize)]
struct CategoryTag {
    slug: String,
    name: String,
}

pub async fn settings(
    State(pool): State<sqlx::PgPool>,
    AuthUser(user): AuthUser,
    Path(slug): Path<String>,
) -> Result<Response, AppError> {
    let row = sqlx::query(
        r#"
        select
            p.id::text as id,
            p.slug,
            p.title,
            p.summary,
            p.description,
            p.project_type,
            p.visibility,
            p.status,
            p.license,
            p.download_count,
            p.monetization_enabled,
            p.creator_share,
            p.icon_key,
            p.owner_id::text as owner_id,
            u.username as owner,
            (
                select r.action from project_reviews r
                where r.project_id = p.id
                order by r.created_at desc limit 1
            ) as review_action,
            (
                select r.notes from project_reviews r
                where r.project_id = p.id
                order by r.created_at desc limit 1
            ) as review_notes
        from projects p
        join users u on u.id = p.owner_id
        where p.slug = $1
        "#,
    )
    .bind(&slug)
    .fetch_optional(&pool)
    .await?;

    let Some(row) = row else {
        return Err(AppError::not_found("project not found"));
    };

    let owner_id: String = row.get("owner_id");
    if owner_id != user.id {
        return Err(AppError::forbidden("not your project"));
    }

    let id: String = row.get("id");

    let category_rows = sqlx::query(
        r#"
        select c.slug, c.name
        from project_categories pc
        join categories c on c.id = pc.category_id
        where pc.project_id = $1::uuid
        order by c.ordering
        "#,
    )
    .bind(&id)
    .fetch_all(&pool)
    .await?;

    let categories: Vec<CategoryTag> = category_rows
        .into_iter()
        .map(|row| CategoryTag {
            slug: row.get("slug"),
            name: row.get("name"),
        })
        .collect();

    let icon_key: Option<String> = row.get("icon_key");
    let icon_url = icon_key.map(|_| format!("/projects/{slug}/icon"));

    let review_action: Option<String> = row.get("review_action");
    let review_notes: Option<String> = row.get("review_notes");

    let body = json!({
        "id": id,
        "slug": row.get::<String, _>("slug"),
        "title": row.get::<String, _>("title"),
        "summary": row.get::<String, _>("summary"),
        "description": row.get::<String, _>("description"),
        "project_type": row.get::<String, _>("project_type"),
        "visibility": row.get::<String, _>("visibility"),
        "status": row.get::<String, _>("status"),
        "license": row.get::<String, _>("license"),
        "download_count": row.get::<i64, _>("download_count"),
        "monetization_enabled": row.get::<bool, _>("monetization_enabled"),
        "creator_share": row.get::<i32, _>("creator_share"),
        "owner": row.get::<String, _>("owner"),
        "icon_url": icon_url,
        "categories": categories,
        "review": review_action.map(|action| json!({
            "action": action,
            "notes": review_notes.unwrap_or_default(),
        })),
    });

    Ok((StatusCode::OK, Json(body)).into_response())
}
