use axum::routing::get;
use axum::{Json, Router, extract::State, http::StatusCode};
use serde_json::{Value, json};

use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().route("/health", get(health))
}

async fn health(State(pool): State<sqlx::PgPool>) -> (StatusCode, Json<Value>) {
    match sqlx::query("SELECT 1").execute(&pool).await {
        Ok(_) => (StatusCode::OK, Json(json!({ "status": "ok" }))),
        Err(_) => (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({ "status": "error", "database": "unreachable" })),
        ),
    }
}
