use axum::response::{IntoResponse, Response};
use axum::{Json, extract::State, http::StatusCode};
use axum_extra::extract::cookie::CookieJar;
use serde::Deserialize;
use serde_json::json;
use sqlx::Row;
use std::sync::OnceLock;

use crate::error::error;
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
) -> Response {
    let email = body.email.trim();

    if email.is_empty() || body.password.is_empty() {
        return error(StatusCode::BAD_REQUEST, "email and password are required").into_response();
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
    .await;

    let row = match row {
        Ok(row) => row,
        Err(_) => {
            return error(StatusCode::INTERNAL_SERVER_ERROR, "could not process login")
                .into_response();
        }
    };

    let stored_hash = row
        .as_ref()
        .and_then(|r| r.get::<Option<String>, _>("password_hash"));
    let hash_to_check = stored_hash.unwrap_or_else(|| dummy_password_hash().to_string());

    let password = body.password.clone();
    let valid =
        match tokio::task::spawn_blocking(move || verify_password(&password, &hash_to_check)).await
        {
            Ok(valid) => valid,
            Err(_) => {
                return error(StatusCode::INTERNAL_SERVER_ERROR, "could not process login")
                    .into_response();
            }
        };

    match row {
        Some(row) if valid => {
            let id: String = row.get("id");
            let username: String = row.get("username");
            let email: String = row.get("email");

            let token = match session::create(&pool, &id).await {
                Ok(token) => token,
                Err(_) => {
                    return error(StatusCode::INTERNAL_SERVER_ERROR, "could not start session")
                        .into_response();
                }
            };

            let jar = jar.add(session::build_cookie(token));
            (
                StatusCode::OK,
                jar,
                Json(json!({ "id": id, "username": username, "email": email })),
            )
                .into_response()
        }
        _ => error(StatusCode::UNAUTHORIZED, "invalid email or password").into_response(),
    }
}

fn dummy_password_hash() -> &'static str {
    static HASH: OnceLock<String> = OnceLock::new();
    HASH.get_or_init(|| {
        hash_password("beacon-dummy-password").expect("failed to build dummy password hash")
    })
}
