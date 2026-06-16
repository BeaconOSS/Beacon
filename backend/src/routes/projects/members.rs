use axum::response::{IntoResponse, Response};
use axum::{Json, extract::Path, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::Row;

use crate::error::AppError;
use crate::extract::AuthUser;
use crate::routes::owner::require_project_owner;

#[derive(Serialize)]
struct Member {
    user_id: String,
    username: String,
    role: String,
}

pub async fn list_members(
    State(pool): State<sqlx::PgPool>,
    AuthUser(user): AuthUser,
    Path(slug): Path<String>,
) -> Result<Response, AppError> {
    let project_id = require_project_owner(&pool, &slug, &user.id).await?;

    let rows = sqlx::query(
        r#"
        select pm.user_id::text as user_id, u.username, pm.role
        from project_members pm
        join users u on u.id = pm.user_id
        where pm.project_id = $1::uuid
        order by (pm.role = 'owner') desc, u.username
        "#,
    )
    .bind(&project_id)
    .fetch_all(&pool)
    .await?;

    let members: Vec<Member> = rows
        .into_iter()
        .map(|row| Member {
            user_id: row.get("user_id"),
            username: row.get("username"),
            role: row.get("role"),
        })
        .collect();

    Ok((StatusCode::OK, Json(json!({ "members": members }))).into_response())
}

#[derive(Deserialize)]
pub struct AddMemberRequest {
    username: String,
}

pub async fn add_member(
    State(pool): State<sqlx::PgPool>,
    AuthUser(user): AuthUser,
    Path(slug): Path<String>,
    Json(body): Json<AddMemberRequest>,
) -> Result<Response, AppError> {
    let project_id = require_project_owner(&pool, &slug, &user.id).await?;

    let username = body.username.trim();
    if username.is_empty() {
        return Err(AppError::bad_request("a username is required"));
    }

    let target = sqlx::query("select id::text as id from users where username = $1")
        .bind(username)
        .fetch_optional(&pool)
        .await?;
    let Some(target) = target else {
        return Err(AppError::not_found("no user with that username"));
    };
    let target_id: String = target.get("id");

    let result =
        sqlx::query("insert into project_members (project_id, user_id, role) values ($1::uuid, $2::uuid, 'member')")
            .bind(&project_id)
            .bind(&target_id)
            .execute(&pool)
            .await;

    if let Err(sqlx::Error::Database(db)) = &result
        && db.is_unique_violation()
    {
        return Err(AppError::conflict("that person is already a member"));
    }
    result?;

    Ok((
        StatusCode::CREATED,
        Json(json!({
            "user_id": target_id,
            "username": username,
            "role": "member",
        })),
    )
        .into_response())
}

pub async fn remove_member(
    State(pool): State<sqlx::PgPool>,
    AuthUser(user): AuthUser,
    Path((slug, user_id)): Path<(String, String)>,
) -> Result<Response, AppError> {
    let project_id = require_project_owner(&pool, &slug, &user.id).await?;

    let result = sqlx::query(
        "delete from project_members \
         where project_id = $1::uuid and user_id = $2::uuid and role <> 'owner'",
    )
    .bind(&project_id)
    .bind(&user_id)
    .execute(&pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::not_found("member not found"));
    }

    Ok(StatusCode::NO_CONTENT.into_response())
}
