use std::collections::HashMap;

use axum::extract::Multipart;

use crate::error::AppError;

/// A single field collected from a multipart upload.
pub struct UploadField {
    pub file_name: Option<String>,
    pub content_type: Option<String>,
    pub bytes: Vec<u8>,
}

/// The fields of a multipart upload, keyed by field name.
///
/// Reads every named field fully into memory so handlers can pull the fields
/// they need and run their own validation. Later fields with a duplicate name
/// overwrite earlier ones, matching the assign-last-wins behavior of a manual
/// `next_field` loop.
pub struct UploadForm {
    fields: HashMap<String, UploadField>,
}

impl UploadForm {
    /// Drain a multipart body into its named fields.
    ///
    /// Returns `bad_request("invalid upload")` if the stream is malformed.
    pub async fn collect(mut multipart: Multipart) -> Result<Self, AppError> {
        let mut fields = HashMap::new();
        loop {
            let field = match multipart.next_field().await {
                Ok(Some(field)) => field,
                Ok(None) => break,
                Err(_) => return Err(AppError::bad_request("invalid upload")),
            };

            let Some(name) = field.name().map(|n| n.to_string()) else {
                let _ = field.bytes().await;
                continue;
            };
            let file_name = field.file_name().map(|f| f.to_string());
            let content_type = field.content_type().map(|c| c.to_string());
            let bytes = match field.bytes().await {
                Ok(bytes) => bytes.to_vec(),
                Err(_) => return Err(AppError::bad_request("invalid upload")),
            };

            fields.insert(
                name,
                UploadField {
                    file_name,
                    content_type,
                    bytes,
                },
            );
        }

        Ok(Self { fields })
    }

    /// Read a field's value as UTF-8 text, or an empty string if absent.
    pub fn text(&self, name: &str) -> String {
        self.fields
            .get(name)
            .map(|field| String::from_utf8_lossy(&field.bytes).into_owned())
            .unwrap_or_default()
    }

    /// Remove and return a field by name, taking ownership of its bytes.
    pub fn take(&mut self, name: &str) -> Option<UploadField> {
        self.fields.remove(name)
    }
}

/// Encode a byte slice as a lowercase hexadecimal string.
pub fn hex_encode(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push_str(&format!("{byte:02x}"));
    }
    out
}

/// Reduce an uploaded filename to a safe base name.
///
/// Strips any directory components and keeps only ASCII alphanumerics plus
/// `.`, `-`, and `_`.
pub fn sanitize_filename(filename: &str) -> String {
    let base = filename.rsplit(['/', '\\']).next().unwrap_or(filename);
    base.chars()
        .filter(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '.' | '-' | '_'))
        .collect()
}
