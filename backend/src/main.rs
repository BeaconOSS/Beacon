use axum::{Json, Router, routing::get};
use serde_json::{Value, json};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/health", get(health));

    let addr = std::env::var("BEACON_ADDR").unwrap_or_else(|_| "127.0.0.1:3000".to_string());

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap_or_else(|_| panic!("failed to bind to {addr}"));

    println!("backend on http://{addr}");

    axum::serve(listener, app)
        .await
        .expect("server error");
}

async fn health() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}
