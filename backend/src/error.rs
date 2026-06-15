use axum::{Json, http::StatusCode};
use serde_json::{Value, json};

pub fn error(status: StatusCode, message: &str) -> (StatusCode, Json<Value>) {
    (status, Json(json!({ "error": message })))
}
