use axum::{Json, Router, extract::State, http::StatusCode, routing::get};
use serde_json::{Value, json};
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("failed to connect to database");

    let frontend_url =
        std::env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:3001".to_string());

    let cors = CorsLayer::new().allow_origin(
        frontend_url
            .parse::<axum::http::HeaderValue>()
            .expect("invalid FRONTEND_URL"),
    );

    let app = Router::new()
        .route("/health", get(health))
        .layer(cors)
        .with_state(pool);

    let addr = std::env::var("BEACON_ADDR").unwrap_or_else(|_| "127.0.0.1:3000".to_string());

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap_or_else(|_| panic!("failed to bind to {addr}"));

    println!("backend on http://{addr}");

    axum::serve(listener, app)
        .await
        .expect("server error");
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
