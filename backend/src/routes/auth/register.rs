use axum::response::{IntoResponse, Response};
use axum::{Json, extract::State, http::StatusCode};
use axum_extra::extract::cookie::CookieJar;
use serde::Deserialize;
use serde_json::json;
use sqlx::Row;

use crate::error::error;
use crate::password::hash_password;
use crate::session;

#[derive(Deserialize)]
pub struct RegisterRequest {
    username: String,
    email: String,
    password: String,
}

pub async fn register(
    State(pool): State<sqlx::PgPool>,
    jar: CookieJar,
    Json(body): Json<RegisterRequest>,
) -> Response {
    let username = body.username.trim();
    let email = body.email.trim();

    if username.is_empty() || email.is_empty() {
        return error(StatusCode::BAD_REQUEST, "username and email are required").into_response();
    }
    if !email.contains('@') {
        return error(StatusCode::BAD_REQUEST, "a valid email is required").into_response();
    }
    if body.password.len() < 8 {
        return error(
            StatusCode::BAD_REQUEST,
            "password must be at least 8 characters",
        )
        .into_response();
    }

    let password = body.password.clone();
    let password_hash = match tokio::task::spawn_blocking(move || hash_password(&password)).await {
        Ok(Ok(hash)) => hash,
        _ => {
            return error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "could not process password",
            )
            .into_response();
        }
    };

    let row = sqlx::query(
        r#"
        with new_user as (
            insert into users (username, email)
            values ($1, $2)
            returning id, username, email
        ), new_identity as (
            insert into user_identities (user_id, provider, password_hash)
            select id, 'password', $3 from new_user
        )
        select id::text as id, username, email from new_user
        "#,
    )
    .bind(username)
    .bind(email)
    .bind(&password_hash)
    .fetch_one(&pool)
    .await;

    match row {
        Ok(row) => {
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
                StatusCode::CREATED,
                jar,
                Json(json!({ "id": id, "username": username, "email": email })),
            )
                .into_response()
        }
        Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() => {
            error(StatusCode::CONFLICT, "username or email is already taken").into_response()
        }
        Err(_) => error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "could not create account",
        )
        .into_response(),
    }
}
