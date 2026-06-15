use sqlx::postgres::PgPoolOptions;

mod error;
mod password;
mod routes;
mod session;
mod state;

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("failed to connect to database");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("failed to run database migrations");

    let frontend_url =
        std::env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:3001".to_string());

    let app = routes::router(pool, &frontend_url);

    let addr = std::env::var("BEACON_ADDR").unwrap_or_else(|_| "127.0.0.1:3000".to_string());

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap_or_else(|_| panic!("failed to bind to {addr}"));

    println!("backend on http://{addr}");

    axum::serve(listener, app).await.expect("server error");
}
