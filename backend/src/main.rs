use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::EnvFilter;

mod analyzer;
mod config;
mod error;
mod extract;
mod password;
mod routes;
mod session;
mod state;
mod storage;

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info,tower_http=debug")),
        )
        .init();

    let config = config::Config::from_env();

    let pool = PgPoolOptions::new()
        .connect(&config.database_url)
        .await
        .expect("failed to connect to database");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("failed to run database migrations");

    let storage = storage::Storage::connect(&config.s3)
        .await
        .expect("failed to initialise object storage");

    let app = routes::router(pool, storage, &config);
    let addr = &config.addr;
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap_or_else(|_| panic!("failed to bind to {addr}"));

    tracing::info!("backend on http://{addr}");

    axum::serve(listener, app).await.expect("server error");
}
