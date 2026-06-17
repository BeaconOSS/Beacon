pub struct Config {
    pub database_url: String,
    pub frontend_url: String,
    pub addr: String,
    pub analyzer_url: Option<String>,
    pub rate_limit_enabled: bool,
    pub s3: S3Config,
    pub oauth: OauthConfig,
}

pub struct S3Config {
    pub endpoint: String,
    pub region: String,
    pub bucket: String,
    pub access_key: String,
    pub secret_key: String,
}

pub struct OauthProvider {
    pub client_id: String,
    pub client_secret: String,
}

pub struct OauthConfig {
    pub github: Option<OauthProvider>,
    pub discord: Option<OauthProvider>,
    pub redirect_base: String,
    pub turnstile_secret: Option<String>,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            database_url: std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            frontend_url: std::env::var("FRONTEND_URL")
                .unwrap_or_else(|_| "http://localhost:3001".to_string()),
            addr: std::env::var("BEACON_ADDR").unwrap_or_else(|_| "127.0.0.1:3000".to_string()),
            analyzer_url: std::env::var("ANALYZER_URL").ok().filter(|s| !s.is_empty()),
            rate_limit_enabled: std::env::var("RATE_LIMIT_ENABLED")
                .map(|value| !matches!(value.trim(), "false" | "0"))
                .unwrap_or(true),
            s3: S3Config::from_env(),
            oauth: OauthConfig::from_env(),
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

impl OauthConfig {
    pub fn from_env() -> Self {
        Self {
            github: OauthProvider::from_env("GITHUB_CLIENT_ID", "GITHUB_CLIENT_SECRET"),
            discord: OauthProvider::from_env("DISCORD_CLIENT_ID", "DISCORD_CLIENT_SECRET"),
            redirect_base: std::env::var("OAUTH_REDIRECT_BASE")
                .unwrap_or_else(|_| "http://localhost:3000".to_string()),
            turnstile_secret: std::env::var("TURNSTILE_SECRET")
                .ok()
                .filter(|s| !s.is_empty()),
        }
    }
}

impl OauthProvider {
    fn from_env(id_key: &str, secret_key: &str) -> Option<Self> {
        match (std::env::var(id_key), std::env::var(secret_key)) {
            (Ok(client_id), Ok(client_secret))
                if !client_id.is_empty() && !client_secret.is_empty() =>
            {
                Some(Self {
                    client_id,
                    client_secret,
                })
            }
            _ => None,
        }
    }
}
