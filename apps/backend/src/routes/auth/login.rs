use axum::response::{IntoResponse, Response};
use axum::{Json, extract::State, http::StatusCode};
use axum_extra::extract::cookie::CookieJar;
use serde::Deserialize;
use serde_json::json;
use sqlx::Row;
use std::sync::OnceLock;

use crate::error::AppError;
use crate::password::{hash_password, verify_password};
use crate::session;

#[derive(Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

pub async fn login(
    State(pool): State<sqlx::PgPool>,
    jar: CookieJar,
    Json(body): Json<LoginRequest>,
) -> Result<Response, AppError> {
    let email = body.email.trim();

    if email.is_empty() || body.password.is_empty() {
        return Err(AppError::bad_request("email and password are required"));
    }

    let row = sqlx::query(
        r#"
        select u.id::text as id, u.username, u.email, i.password_hash
        from users u
        join user_identities i on i.user_id = u.id and i.provider = 'password'
        where u.email = $1
        "#,
    )
    .bind(email)
    .fetch_optional(&pool)
    .await
    .map_err(|_| AppError::internal("could not process login"))?;

    let stored_hash = row
        .as_ref()
        .and_then(|r| r.get::<Option<String>, _>("password_hash"));
    let hash_to_check = stored_hash.unwrap_or_else(|| dummy_password_hash().to_string());

    let password = body.password.clone();
    let valid = tokio::task::spawn_blocking(move || verify_password(&password, &hash_to_check))
        .await
        .map_err(|_| AppError::internal("could not process login"))?;

    match row {
        Some(row) if valid => {
            let id: String = row.get("id");
            let username: String = row.get("username");
            let email: String = row.get("email");

            let token = session::create(&pool, &id)
                .await
                .map_err(|_| AppError::internal("could not start session"))?;

            let jar = jar.add(session::build_cookie(token));
            Ok((
                StatusCode::OK,
                jar,
                Json(json!({ "id": id, "username": username, "email": email })),
            )
                .into_response())
        }
        _ => Err(AppError::unauthorized("invalid email or password")),
    }
}

fn dummy_password_hash() -> &'static str {
    static HASH: OnceLock<String> = OnceLock::new();
    HASH.get_or_init(|| {
        hash_password("beacon-dummy-password").expect("failed to build dummy password hash")
    })
}
