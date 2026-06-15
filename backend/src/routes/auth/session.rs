use axum::response::{IntoResponse, Response};
use axum::{Json, extract::State, http::StatusCode};
use axum_extra::extract::cookie::CookieJar;
use serde_json::json;

use crate::error::error;
use crate::session;

pub async fn me(State(pool): State<sqlx::PgPool>, jar: CookieJar) -> Response {
    let Some(token) = jar.get(session::SESSION_COOKIE).map(|c| c.value().to_string()) else {
        return error(StatusCode::UNAUTHORIZED, "not signed in").into_response();
    };

    match session::lookup(&pool, &token).await {
        Ok(Some(user)) => (
            StatusCode::OK,
            Json(json!({ "id": user.id, "username": user.username, "email": user.email })),
        )
            .into_response(),
        Ok(None) => error(StatusCode::UNAUTHORIZED, "not signed in").into_response(),
        Err(_) => {
            error(StatusCode::INTERNAL_SERVER_ERROR, "could not read session").into_response()
        }
    }
}

pub async fn logout(State(pool): State<sqlx::PgPool>, jar: CookieJar) -> Response {
    if let Some(token) = jar.get(session::SESSION_COOKIE).map(|c| c.value().to_string()) {
        let _ = session::delete(&pool, &token).await;
    }

    let jar = jar.add(session::clear_cookie());
    (StatusCode::OK, jar, Json(json!({ "status": "signed out" }))).into_response()
}
