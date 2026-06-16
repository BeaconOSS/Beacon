use std::collections::HashMap;
use std::io::Read;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sqlx::{PgPool, Row};

use crate::analyzer::AnalyzerClient;
use crate::storage::Storage;

const MAX_ENTRIES: usize = 20_000;
const MAX_TOTAL_UNCOMPRESSED: u64 = 512 * 1024 * 1024;
pub const MAX_INNER_FILE_BYTES: u64 = 8 * 1024 * 1024;
const MAX_HASH_BYTES: u64 = 64 * 1024 * 1024;
const READ_CHUNK: usize = 64 * 1024;
const MAX_DIFF_FILES: usize = 2_000;
const BACKFILL_LIMIT: i64 = 50;

#[derive(Clone, Serialize, Deserialize)]
pub struct FileEntry {
    pub path: String,
    pub size: i64,
    pub sha256: String,
    pub kind: String,
}

pub fn build_file_index(bytes: &[u8]) -> (Vec<FileEntry>, bool) {
    let reader = std::io::Cursor::new(bytes);
    let mut archive = match zip::ZipArchive::new(reader) {
        Ok(archive) => archive,
        Err(_) => return (Vec::new(), true),
    };

    let mut entries: Vec<FileEntry> = Vec::new();
    let mut truncated = false;
    let mut total_uncompressed: u64 = 0;

    for i in 0..archive.len() {
        if entries.len() >= MAX_ENTRIES {
            truncated = true;
            break;
        }

        let mut file = match archive.by_index(i) {
            Ok(file) => file,
            Err(_) => {
                truncated = true;
                continue;
            }
        };

        if file.is_dir() {
            continue;
        }

        let path = file.name().to_string();
        let declared_size = file.size();

        if total_uncompressed.saturating_add(declared_size) > MAX_TOTAL_UNCOMPRESSED {
            truncated = true;
            break;
        }

        let (size, sha256) = if declared_size > MAX_HASH_BYTES {
            truncated = true;
            (declared_size, String::new())
        } else {
            let mut hasher = Sha256::new();
            let mut read_total: u64 = 0;
            let mut buf = [0u8; READ_CHUNK];
            let mut failed = false;
            loop {
                match file.read(&mut buf) {
                    Ok(0) => break,
                    Ok(n) => {
                        read_total += n as u64;
                        if read_total > MAX_HASH_BYTES {
                            truncated = true;
                            failed = true;
                            break;
                        }
                        hasher.update(&buf[..n]);
                    }
                    Err(_) => {
                        truncated = true;
                        failed = true;
                        break;
                    }
                }
            }
            if failed {
                (read_total, String::new())
            } else {
                (read_total, hex_encode(&hasher.finalize()))
            }
        };

        total_uncompressed = total_uncompressed.saturating_add(size);
        let kind = classify_kind(&path);
        entries.push(FileEntry {
            path,
            size: size as i64,
            sha256,
            kind,
        });
    }

    entries.sort_by(|a, b| a.path.cmp(&b.path));
    (entries, truncated)
}

fn classify_kind(path: &str) -> String {
    let lower = path.to_ascii_lowercase();
    let name = lower.rsplit('/').next().unwrap_or(&lower);
    let ext = name.rsplit_once('.').map(|(_, e)| e).unwrap_or("");
    let has = |segment: &str| {
        lower
            .split('/')
            .any(|part| part == segment || part == format!("{segment}s"))
    };

    if name == "manifest.json" {
        return "manifest".to_string();
    }
    if name.ends_with(".geo.json") || has("model") || has("geometry") {
        return "model".to_string();
    }
    if has("entity") {
        return "entity".to_string();
    }
    if has("block") {
        return "block".to_string();
    }
    if has("item") {
        return "item".to_string();
    }
    if has("texture") || matches!(ext, "png" | "tga" | "jpg" | "jpeg") {
        return "texture".to_string();
    }
    if has("animation_controller") || has("animation") {
        return "animation".to_string();
    }
    if has("render_controller") {
        return "render_controller".to_string();
    }
    if has("particle") {
        return "particle".to_string();
    }
    if has("sound") || matches!(ext, "ogg" | "fsb" | "wav" | "mp3") {
        return "sound".to_string();
    }
    if has("function") || ext == "mcfunction" {
        return "function".to_string();
    }
    if has("text") || ext == "lang" {
        return "lang".to_string();
    }
    if ext == "material" {
        return "material".to_string();
    }
    "other".to_string()
}

pub async fn store_file_index(
    pool: &PgPool,
    file_id: &str,
    entries: &[FileEntry],
    truncated: bool,
) {
    let json = serde_json::to_string(entries).unwrap_or_else(|_| "[]".to_string());
    let result = sqlx::query(
        "insert into version_file_analysis (file_id, file_index, truncated, analyzed_at) \
         values ($1::uuid, $2::jsonb, $3, now()) \
         on conflict (file_id) do update set \
           file_index = excluded.file_index, \
           truncated = version_file_analysis.truncated or excluded.truncated",
    )
    .bind(file_id)
    .bind(&json)
    .bind(truncated)
    .execute(pool)
    .await;

    if let Err(err) = result {
        tracing::warn!("could not store file index for file {file_id}: {err}");
    }
}

#[derive(Serialize)]
pub struct PackDiffEntry {
    pub path: String,
    pub kind: String,
    pub status: String,
    pub old_size: Option<i64>,
    pub new_size: Option<i64>,
}

#[derive(Serialize)]
pub struct PackDiffKind {
    pub kind: String,
    pub added: i64,
    pub removed: i64,
    pub modified: i64,
}

#[derive(Serialize)]
pub struct PackDiff {
    pub added: i64,
    pub removed: i64,
    pub modified: i64,
    pub unchanged: i64,
    pub by_kind: Vec<PackDiffKind>,
    pub files: Vec<PackDiffEntry>,
    pub files_truncated: bool,
}

pub fn diff_indexes(old: &[FileEntry], new: &[FileEntry]) -> PackDiff {
    let old_map: HashMap<&str, &FileEntry> = old.iter().map(|e| (e.path.as_str(), e)).collect();
    let new_map: HashMap<&str, &FileEntry> = new.iter().map(|e| (e.path.as_str(), e)).collect();

    let mut added = 0i64;
    let mut removed = 0i64;
    let mut modified = 0i64;
    let mut unchanged = 0i64;
    let mut kinds: HashMap<String, PackDiffKind> = HashMap::new();
    let mut files: Vec<PackDiffEntry> = Vec::new();

    let mut bump = |kind: &str, status: &str| {
        let entry = kinds
            .entry(kind.to_string())
            .or_insert_with(|| PackDiffKind {
                kind: kind.to_string(),
                added: 0,
                removed: 0,
                modified: 0,
            });
        match status {
            "added" => entry.added += 1,
            "removed" => entry.removed += 1,
            "modified" => entry.modified += 1,
            _ => {}
        }
    };

    for entry in new {
        match old_map.get(entry.path.as_str()) {
            None => {
                added += 1;
                bump(&entry.kind, "added");
                files.push(PackDiffEntry {
                    path: entry.path.clone(),
                    kind: entry.kind.clone(),
                    status: "added".to_string(),
                    old_size: None,
                    new_size: Some(entry.size),
                });
            }
            Some(old_entry) => {
                if old_entry.sha256 != entry.sha256 || old_entry.sha256.is_empty() {
                    modified += 1;
                    bump(&entry.kind, "modified");
                    files.push(PackDiffEntry {
                        path: entry.path.clone(),
                        kind: entry.kind.clone(),
                        status: "modified".to_string(),
                        old_size: Some(old_entry.size),
                        new_size: Some(entry.size),
                    });
                } else {
                    unchanged += 1;
                }
            }
        }
    }

    for entry in old {
        if !new_map.contains_key(entry.path.as_str()) {
            removed += 1;
            bump(&entry.kind, "removed");
            files.push(PackDiffEntry {
                path: entry.path.clone(),
                kind: entry.kind.clone(),
                status: "removed".to_string(),
                old_size: Some(entry.size),
                new_size: None,
            });
        }
    }

    files.sort_by(|a, b| a.path.cmp(&b.path));
    let files_truncated = files.len() > MAX_DIFF_FILES;
    files.truncate(MAX_DIFF_FILES);

    let mut by_kind: Vec<PackDiffKind> = kinds.into_values().collect();
    by_kind.sort_by(|a, b| a.kind.cmp(&b.kind));

    PackDiff {
        added,
        removed,
        modified,
        unchanged,
        by_kind,
        files,
        files_truncated,
    }
}

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

fn hex_encode(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push_str(&format!("{byte:02x}"));
    }
    out
}

pub fn read_inner_file(bytes: &[u8], path: &str) -> Result<Option<Vec<u8>>, String> {
    let reader = std::io::Cursor::new(bytes);
    let mut archive = zip::ZipArchive::new(reader).map_err(|e| e.to_string())?;

    let mut file = match archive.by_name(path) {
        Ok(file) => file,
        Err(zip::result::ZipError::FileNotFound) => return Ok(None),
        Err(e) => return Err(e.to_string()),
    };

    if file.is_dir() {
        return Ok(None);
    }
    if file.size() > MAX_INNER_FILE_BYTES {
        return Err("file too large".to_string());
    }

    let mut out = Vec::with_capacity(file.size() as usize);
    let mut buf = [0u8; READ_CHUNK];
    loop {
        match file.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                if out.len() as u64 + n as u64 > MAX_INNER_FILE_BYTES {
                    return Err("file too large".to_string());
                }
                out.extend_from_slice(&buf[..n]);
            }
            Err(e) => return Err(e.to_string()),
        }
    }

    Ok(Some(out))
}

pub fn guess_content_type(path: &str) -> &'static str {
    let ext = path
        .rsplit('/')
        .next()
        .unwrap_or(path)
        .rsplit_once('.')
        .map(|(_, e)| e.to_ascii_lowercase())
        .unwrap_or_default();

    match ext.as_str() {
        "json" | "geo" => "application/json; charset=utf-8",
        "material" | "lang" | "mcfunction" | "txt" | "properties" | "js" | "ts" | "html"
        | "css" | "md" => "text/plain; charset=utf-8",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "tga" => "image/x-tga",
        "ogg" => "audio/ogg",
        "wav" => "audio/wav",
        "mp3" => "audio/mpeg",
        _ => "application/octet-stream",
    }
}
