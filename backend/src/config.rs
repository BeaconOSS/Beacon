pub struct Config {
    pub database_url: String,
    pub frontend_url: String,
    pub addr: String,
    pub s3: S3Config,
}

pub struct S3Config {
    pub endpoint: String,
    pub region: String,
    pub bucket: String,
    pub access_key: String,
    pub secret_key: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            database_url: std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            frontend_url: std::env::var("FRONTEND_URL")
                .unwrap_or_else(|_| "http://localhost:3001".to_string()),
            addr: std::env::var("BEACON_ADDR").unwrap_or_else(|_| "127.0.0.1:3000".to_string()),
            s3: S3Config::from_env(),
        }
    }
}

impl S3Config {
    pub fn from_env() -> Self {
        Self {
            endpoint: std::env::var("S3_ENDPOINT").expect("S3_ENDPOINT must be set"),
            region: std::env::var("S3_REGION").unwrap_or_else(|_| "us-east-1".to_string()),
            bucket: std::env::var("S3_BUCKET").expect("S3_BUCKET must be set"),
            access_key: std::env::var("S3_ACCESS_KEY").expect("S3_ACCESS_KEY must be set"),
            secret_key: std::env::var("S3_SECRET_KEY").expect("S3_SECRET_KEY must be set"),
        }
    }
}
