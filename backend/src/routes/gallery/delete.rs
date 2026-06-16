use axum::response::{IntoResponse, Response};
use axum::{extract::Path, extract::State, http::StatusCode};
use sqlx::Row;

use crate::error::AppError;
use crate::extract::AuthUser;
use crate::storage::Storage;

pub async fn delete_gallery_image(
    State(pool): State<sqlx::PgPool>,
    State(storage): State<Storage>,
    AuthUser(user): AuthUser,
    Path((slug, image_id)): Path<(String, String)>,
) -> Result<Response, AppError> {
    let user_id = user.id;

    let row = sqlx::query(
        r#"
        select g.id::text as id, g.storage_key, p.owner_id::text as owner_id, p.status
        from gallery_images g
        join projects p on p.id = g.project_id
        where p.slug = $1 and g.id = $2::uuid
        "#,
    )
    .bind(&slug)
    .bind(&image_id)
    .fetch_optional(&pool)
    .await?;

    let Some(row) = row else {
        return Err(AppError::not_found("image not found"));
    };
    let storage_key: String = row.get("storage_key");
    let owner_id: String = row.get("owner_id");

    if owner_id != user_id {
        return Err(AppError::forbidden("not your project"));
    }

    let status: String = row.get("status");
    if status == "in_review" {
        return Err(AppError::conflict(
            "this project is locked while it is under review",
        ));
    }

    sqlx::query("delete from gallery_images where id = $1::uuid")
        .bind(&image_id)
        .execute(&pool)
        .await?;

    let _ = storage.delete(&storage_key).await;

    Ok(StatusCode::NO_CONTENT.into_response())
}
