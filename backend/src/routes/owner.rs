use sqlx::{PgPool, Row};

use crate::error::AppError;

pub(crate) async fn require_project_owner(
    pool: &PgPool,
    slug: &str,
    user_id: &str,
) -> Result<String, AppError> {
    let project = sqlx::query(
        "select id::text as id, owner_id::text as owner_id from projects where slug = $1",
    )
    .bind(slug)
    .fetch_optional(pool)
    .await?;

    let Some(project) = project else {
        return Err(AppError::not_found("project not found"));
    };

    let owner_id: String = project.get("owner_id");
    if owner_id != user_id {
        return Err(AppError::forbidden("not your project"));
    }

    Ok(project.get("id"))
}

pub(crate) async fn ensure_not_in_review(pool: &PgPool, project_id: &str) -> Result<(), AppError> {
    let status: String = sqlx::query("select status from projects where id = $1::uuid")
        .bind(project_id)
        .fetch_one(pool)
        .await?
        .get("status");

    if status == "in_review" {
        return Err(AppError::conflict(
            "this project is locked while it is under review",
        ));
    }

    Ok(())
}

pub(crate) async fn has_pending_changes(pool: &PgPool, project_id: &str) -> Result<bool, AppError> {
    let row = sqlx::query(
        "select (p.published_at is not null and (\
             p.title is distinct from p.published_title \
             or p.summary is distinct from p.published_summary \
             or p.description is distinct from p.published_description \
             or p.license is distinct from p.published_license \
             or p.icon_key is distinct from p.published_icon_key \
             or exists(select category_id from project_categories \
                       where project_id = p.id \
                       except select category_id from project_published_categories \
                       where project_id = p.id) \
             or exists(select category_id from project_published_categories \
                       where project_id = p.id \
                       except select category_id from project_categories \
                       where project_id = p.id) \
         )) as has_pending \
         from projects p where p.id = $1::uuid",
    )
    .bind(project_id)
    .fetch_one(pool)
    .await?;

    Ok(row.get("has_pending"))
}
