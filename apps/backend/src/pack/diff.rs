use std::collections::HashMap;

use serde::Serialize;

use super::index::FileEntry;

const MAX_DIFF_FILES: usize = 2_000;

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
