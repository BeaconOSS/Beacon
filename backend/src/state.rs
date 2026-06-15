use axum::extract::FromRef;
use sqlx::PgPool;

#[derive(Clone)]
pub struct GithubOauth {
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Clone)]
pub struct DiscordOauth {
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub github: Option<GithubOauth>,
    pub discord: Option<DiscordOauth>,
    pub redirect_base: String,
    pub frontend_url: String,
}

impl AppState {
    pub fn from_env(pool: PgPool, frontend_url: &str) -> Self {
        let github = match (
            std::env::var("GITHUB_CLIENT_ID"),
            std::env::var("GITHUB_CLIENT_SECRET"),
        ) {
            (Ok(client_id), Ok(client_secret))
                if !client_id.is_empty() && !client_secret.is_empty() =>
            {
                Some(GithubOauth {
                    client_id,
                    client_secret,
                })
            }
            _ => None,
        };

        let discord = match (
            std::env::var("DISCORD_CLIENT_ID"),
            std::env::var("DISCORD_CLIENT_SECRET"),
        ) {
            (Ok(client_id), Ok(client_secret))
                if !client_id.is_empty() && !client_secret.is_empty() =>
            {
                Some(DiscordOauth {
                    client_id,
                    client_secret,
                })
            }
            _ => None,
        };

        let redirect_base = std::env::var("OAUTH_REDIRECT_BASE")
            .unwrap_or_else(|_| "http://localhost:3000".to_string());

        Self {
            pool,
            github,
            discord,
            redirect_base,
            frontend_url: frontend_url.to_string(),
        }
    }
}

impl FromRef<AppState> for PgPool {
    fn from_ref(state: &AppState) -> Self {
        state.pool.clone()
    }
}
