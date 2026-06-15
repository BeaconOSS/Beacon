use axum::{Json, Router, routing::get};
use serde_json::{Value, json};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/health", get(health));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("failed to bind to 127.0.0.1:3000");

    println!("backend on http://127.0.0.1:3000");

    axum::serve(listener, app)
        .await
        .expect("server error");
}

async fn health() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}
