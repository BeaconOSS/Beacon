use axum::response::{IntoResponse, Response};
use axum::{Json, extract::State, http::StatusCode};
use axum_extra::extract::cookie::CookieJar;
use serde::Deserialize;
use serde_json::json;
use sqlx::Row;

use crate::error::AppError;
use crate::password::hash_password;
use crate::routes::auth::oauth::unique_username;
use crate::routes::auth::turnstile;
use crate::session;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct RegisterRequest {
    email: String,
    password: String,
    #[serde(default)]
    turnstile_token: Option<String>,
}

pub async fn register(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(body): Json<RegisterRequest>,
) -> Result<Response, AppError> {
    let email = body.email.trim();

    if !session::allow_registration() {
        return Err(AppError::forbidden("registration is currently closed"));
    }

    if email.is_empty() {
        return Err(AppError::bad_request("email is required"));
    }
    if !email.contains('@') {
        return Err(AppError::bad_request("a valid email is required"));
    }
    if body.password.len() < 8 {
        return Err(AppError::bad_request(
            "password must be at least 8 characters",
        ));
    }

    if let Some(secret) = state.turnstile_secret.as_ref() {
        let token = body.turnstile_token.as_deref().unwrap_or("");
        if token.is_empty() {
            return Err(AppError::bad_request("please complete the captcha"));
        }
        if !turnstile::verify(secret, token).await {
            return Err(AppError::bad_request("captcha verification failed"));
        }
    }

    let username = unique_username(&state.pool, email.split('@').next().unwrap_or(""))
        .await
        .map_err(|_| AppError::internal("could not create account"))?;

    let password = body.password.clone();
    let password_hash = match tokio::task::spawn_blocking(move || hash_password(&password)).await {
        Ok(Ok(hash)) => hash,
        _ => return Err(AppError::internal("could not process password")),
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
    .bind(&username)
    .bind(email)
    .bind(&password_hash)
    .fetch_one(&state.pool)
    .await;

    let row = match row {
        Ok(row) => row,
        Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() => {
            return Err(AppError::conflict("email is already taken"));
        }
        Err(_) => return Err(AppError::internal("could not create account")),
    };

    let id: String = row.get("id");
    let username: String = row.get("username");
    let email: String = row.get("email");

    let token = session::create(&state.pool, &id)
        .await
        .map_err(|_| AppError::internal("could not start session"))?;

    let jar = jar.add(session::build_cookie(token));
    Ok((
        StatusCode::CREATED,
        jar,
        Json(json!({ "id": id, "username": username, "email": email })),
    )
        .into_response())
}
