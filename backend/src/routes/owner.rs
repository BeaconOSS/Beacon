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
