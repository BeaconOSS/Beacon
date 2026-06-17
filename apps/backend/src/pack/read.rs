use std::io::Read;

use super::READ_CHUNK;

pub const MAX_INNER_FILE_BYTES: u64 = 8 * 1024 * 1024;

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
