use std::io::Read;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sqlx::PgPool;

use crate::utils::hex_encode;

use super::READ_CHUNK;

const MAX_ENTRIES: usize = 20_000;
const MAX_TOTAL_UNCOMPRESSED: u64 = 512 * 1024 * 1024;
const MAX_HASH_BYTES: u64 = 64 * 1024 * 1024;

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
