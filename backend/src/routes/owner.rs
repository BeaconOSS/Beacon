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
