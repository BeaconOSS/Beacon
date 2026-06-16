use axum::extract::FromRef;
use sqlx::PgPool;

use crate::analyzer::AnalyzerClient;
use crate::config::Config;
use crate::storage::Storage;

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
    pub storage: Storage,
    pub analyzer: AnalyzerClient,
    pub github: Option<GithubOauth>,
    pub discord: Option<DiscordOauth>,
    pub turnstile_secret: Option<String>,
    pub redirect_base: String,
    pub frontend_url: String,
}

impl AppState {
    pub fn new(pool: PgPool, storage: Storage, config: &Config) -> Self {
        let github = config.oauth.github.as_ref().map(|p| GithubOauth {
            client_id: p.client_id.clone(),
            client_secret: p.client_secret.clone(),
        });

        let discord = config.oauth.discord.as_ref().map(|p| DiscordOauth {
            client_id: p.client_id.clone(),
            client_secret: p.client_secret.clone(),
        });

        Self {
            pool,
            storage,
            analyzer: AnalyzerClient::new(config.analyzer_url.clone()),
            github,
            discord,
            turnstile_secret: config.oauth.turnstile_secret.clone(),
            redirect_base: config.oauth.redirect_base.clone(),
            frontend_url: config.frontend_url.clone(),
        }
    }
}

impl FromRef<AppState> for PgPool {
    fn from_ref(state: &AppState) -> Self {
        state.pool.clone()
    }
}

impl FromRef<AppState> for Storage {
    fn from_ref(state: &AppState) -> Self {
        state.storage.clone()
    }
}

impl FromRef<AppState> for AnalyzerClient {
    fn from_ref(state: &AppState) -> Self {
        state.analyzer.clone()
    }
}
