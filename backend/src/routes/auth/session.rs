use axum::response::{IntoResponse, Response};
use axum::{Json, extract::State, http::StatusCode};
use axum_extra::extract::cookie::CookieJar;
use serde_json::json;

use crate::extract::AuthUser;
use crate::session;

pub async fn me(AuthUser(user): AuthUser) -> Response {
    (
        StatusCode::OK,
        Json(json!({ "id": user.id, "username": user.username, "email": user.email })),
    )
        .into_response()
}

pub async fn logout(State(pool): State<sqlx::PgPool>, jar: CookieJar) -> Response {
    if let Some(token) = jar.get(session::SESSION_COOKIE).map(|c| c.value().to_string()) {
        let _ = session::delete(&pool, &token).await;
    }

    let jar = jar.add(session::clear_cookie());
    (StatusCode::OK, jar, Json(json!({ "status": "signed out" }))).into_response()
}
