pub struct Config {
    pub database_url: String,
    pub frontend_url: String,
    pub addr: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            database_url: std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            frontend_url: std::env::var("FRONTEND_URL")
                .unwrap_or_else(|_| "http://localhost:3001".to_string()),
            addr: std::env::var("BEACON_ADDR").unwrap_or_else(|_| "127.0.0.1:3000".to_string()),
        }
    }
}
