use sqlx::{PgPool, Row};

use crate::analyzer::AnalyzerClient;
use crate::storage::Storage;

use super::index::{build_file_index, store_file_index};

const BACKFILL_LIMIT: i64 = 50;

pub async fn backfill(pool: PgPool, storage: Storage, analyzer: AnalyzerClient) {
    let rows = sqlx::query(
        "select f.id::text as file_id, f.storage_key, (a.report is null) as needs_report \
		 from files f \
		 left join version_file_analysis a on a.file_id = f.id \
		 where f.is_primary = true and (a.file_id is null or a.file_index is null) \
		 order by f.created_at desc \
		 limit $1",
    )
    .bind(BACKFILL_LIMIT)
    .fetch_all(&pool)
    .await;

    let rows = match rows {
        Ok(rows) => rows,
        Err(err) => {
            tracing::warn!("pack backfill query failed: {err}");
            return;
        }
    };

    if rows.is_empty() {
        return;
    }

    tracing::info!("pack backfill: {} file(s) to analyze", rows.len());

    for row in rows {
        let file_id: String = row.get("file_id");
        let storage_key: String = row.get("storage_key");
        let needs_report: bool = row.get("needs_report");

        let bytes = match storage.get(&storage_key).await {
            Ok(bytes) => bytes,
            Err(err) => {
                tracing::warn!("pack backfill: could not fetch {storage_key}: {err}");
                continue;
            }
        };

        let index_bytes = bytes.clone();
        let indexed = tokio::task::spawn_blocking(move || build_file_index(&index_bytes)).await;
        if let Ok((entries, truncated)) = indexed {
            store_file_index(&pool, &file_id, &entries, truncated).await;
        }

        if needs_report && analyzer.enabled() {
            analyzer
                .analyze_and_store(&pool, &file_id, "addon", bytes)
                .await;
        }
    }
}
