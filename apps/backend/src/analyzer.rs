use sqlx::PgPool;

const ANALYSIS_SCHEMA_VERSION: i32 = 1;
#[derive(Clone)]
pub struct AnalyzerClient {
    base_url: Option<String>,
    http: reqwest::Client,
}

impl AnalyzerClient {
    pub fn new(base_url: Option<String>) -> Self {
        Self {
            base_url: base_url.filter(|u| !u.trim().is_empty()),
            http: reqwest::Client::new(),
        }
    }

    pub fn enabled(&self) -> bool {
        self.base_url.is_some()
    }

    pub async fn analyze_and_store(
        &self,
        pool: &PgPool,
        file_id: &str,
        suite: &str,
        bytes: Vec<u8>,
    ) {
        let Some(base) = self.base_url.as_deref() else {
            return;
        };
        let url = format!("{}/analyze?suite={suite}", base.trim_end_matches('/'));

        match self.fetch(&url, bytes).await {
            Ok((report, mctools_version)) => {
                store_report(pool, file_id, &report, &mctools_version).await;
            }
            Err(err) => {
                tracing::warn!("pack analysis failed for file {file_id}: {err}");
                store_error(pool, file_id, &err).await;
            }
        }
    }

    async fn fetch(&self, url: &str, bytes: Vec<u8>) -> Result<(String, String), String> {
        let resp = self
            .http
            .post(url)
            .header("content-type", "application/octet-stream")
            .body(bytes)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if !resp.status().is_success() {
            return Err(format!("analyzer returned {}", resp.status()));
        }

        let text = resp.text().await.map_err(|e| e.to_string())?;
        let value: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
        let mctools_version = value
            .get("mctoolsVersion")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        Ok((text, mctools_version))
    }
}

async fn store_report(pool: &PgPool, file_id: &str, report_json: &str, mctools_version: &str) {
    let result = sqlx::query(
        "insert into version_file_analysis \
           (file_id, report, mctools_version, schema_version, analyzed_at, error) \
         values ($1::uuid, $2::jsonb, $3, $4, now(), '') \
         on conflict (file_id) do update set \
           report = excluded.report, \
           mctools_version = excluded.mctools_version, \
           schema_version = excluded.schema_version, \
           analyzed_at = now(), \
           error = ''",
    )
    .bind(file_id)
    .bind(report_json)
    .bind(mctools_version)
    .bind(ANALYSIS_SCHEMA_VERSION)
    .execute(pool)
    .await;

    if let Err(err) = result {
        tracing::warn!("could not store analysis for file {file_id}: {err}");
    }
}

async fn store_error(pool: &PgPool, file_id: &str, message: &str) {
    let result = sqlx::query(
        "insert into version_file_analysis \
           (file_id, report, mctools_version, schema_version, analyzed_at, error) \
         values ($1::uuid, null, '', $2, now(), $3) \
         on conflict (file_id) do update set \
           report = null, \
           analyzed_at = now(), \
           error = excluded.error",
    )
    .bind(file_id)
    .bind(ANALYSIS_SCHEMA_VERSION)
    .bind(message)
    .execute(pool)
    .await;

    if let Err(err) = result {
        tracing::warn!("could not store analysis error for file {file_id}: {err}");
    }
}
