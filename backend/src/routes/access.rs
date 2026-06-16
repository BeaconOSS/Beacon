use axum_extra::extract::cookie::CookieJar;
use sqlx::{PgPool, Row};

use crate::error::AppError;
use crate::session;

pub(crate) async fn project_for_viewer(
    pool: &PgPool,
    jar: &CookieJar,
    slug: &str,
) -> Result<String, AppError> {
    let row = sqlx::query(
        "select id::text as id, owner_id::text as owner_id, visibility, \
         published_at is not null as is_published \
         from projects where slug = $1",
    )
    .bind(slug)
    .fetch_optional(pool)
    .await?;

    let Some(row) = row else {
        return Err(AppError::not_found("project not found"));
    };

    let id: String = row.get("id");
    let owner_id: String = row.get("owner_id");
    let visibility: String = row.get("visibility");
    let is_published: bool = row.get("is_published");

    let viewer = match jar.get(session::SESSION_COOKIE) {
        Some(cookie) => session::lookup(pool, cookie.value()).await.ok().flatten(),
        None => None,
    };

    let is_owner = viewer.as_ref().map(|user| user.id.as_str()) == Some(owner_id.as_str());
    let is_moderator = viewer
        .as_ref()
        .is_some_and(|user| user.role == "moderator" || user.role == "admin");
    let publicly_visible = is_published && visibility != "private";

    if publicly_visible || is_owner || is_moderator {
        Ok(id)
    } else {
        Err(AppError::not_found("project not found"))
    }
}
