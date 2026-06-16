use axum::response::{IntoResponse, Response};
use axum::{Json, extract::Path, extract::State, http::StatusCode};
use axum_extra::extract::cookie::CookieJar;
use serde_json::json;
use sqlx::Row;

use crate::error::AppError;
use crate::extract::AuthUser;
use crate::routes::access::project_for_viewer;

pub async fn toggle_heart(
    State(pool): State<sqlx::PgPool>,
    jar: CookieJar,
    AuthUser(user): AuthUser,
    Path(slug): Path<String>,
) -> Result<Response, AppError> {
    let project_id = project_for_viewer(&pool, &jar, &slug).await?;

    let deleted = sqlx::query(
        "delete from project_hearts where project_id = $1::uuid and user_id = $2::uuid",
    )
    .bind(&project_id)
    .bind(&user.id)
    .execute(&pool)
    .await?;

    let hearted = if deleted.rows_affected() == 0 {
        sqlx::query(
            "insert into project_hearts (project_id, user_id) values ($1::uuid, $2::uuid) \
             on conflict do nothing",
        )
        .bind(&project_id)
        .bind(&user.id)
        .execute(&pool)
        .await?;
        true
    } else {
        false
    };

    let count_row =
        sqlx::query("select count(*) as count from project_hearts where project_id = $1::uuid")
            .bind(&project_id)
            .fetch_one(&pool)
            .await?;
    let heart_count: i64 = count_row.get("count");

    Ok((
        StatusCode::OK,
        Json(json!({ "hearted": hearted, "heart_count": heart_count })),
    )
        .into_response())
}

pub async fn toggle_save(
    State(pool): State<sqlx::PgPool>,
    jar: CookieJar,
    AuthUser(user): AuthUser,
    Path(slug): Path<String>,
) -> Result<Response, AppError> {
    let project_id = project_for_viewer(&pool, &jar, &slug).await?;

    let deleted =
        sqlx::query("delete from project_saves where project_id = $1::uuid and user_id = $2::uuid")
            .bind(&project_id)
            .bind(&user.id)
            .execute(&pool)
            .await?;

    let saved = if deleted.rows_affected() == 0 {
        sqlx::query(
            "insert into project_saves (project_id, user_id) values ($1::uuid, $2::uuid) \
             on conflict do nothing",
        )
        .bind(&project_id)
        .bind(&user.id)
        .execute(&pool)
        .await?;
        true
    } else {
        false
    };

    Ok((StatusCode::OK, Json(json!({ "saved": saved }))).into_response())
}
