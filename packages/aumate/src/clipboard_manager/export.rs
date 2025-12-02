//! Export and import clipboard history

use std::fs;
use std::path::Path;

use chrono::Utc;
use serde::{Deserialize, Serialize};

use super::db::ClipboardDb;
use super::entry::{CategoryFilter, ClipboardContent, ClipboardEntry, Tag};

/// Export format version
const EXPORT_VERSION: &str = "1.0";

/// Export format options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ExportFormat {
    /// JSON format (default)
    #[default]
    Json,
}

/// Exported clipboard data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportData {
    /// Export format version
    pub version: String,
    /// Timestamp of export
    pub exported_at: String,
    /// Number of entries exported
    pub entry_count: usize,
    /// Exported entries (sensitive entries excluded)
    pub entries: Vec<ExportEntry>,
    /// Exported tags
    pub tags: Vec<ExportTag>,
}

/// Exported entry (without sensitive data)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportEntry {
    /// Original entry ID
    pub id: String,
    /// Content type
    pub content_type: String,
    /// Text content (for text entries)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Base64-encoded image data (for image entries)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_base64: Option<String>,
    /// Image dimensions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_height: Option<u32>,
    /// File paths (for file entries)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<String>>,
    /// Preview text
    pub preview: String,
    /// Whether this is a favorite
    pub is_favorite: bool,
    /// Whether this is pinned
    pub is_pinned: bool,
    /// Creation timestamp
    pub created_at: String,
    /// Tag IDs
    pub tag_ids: Vec<String>,
}

/// Exported tag
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportTag {
    pub id: String,
    pub name: String,
    pub color: String,
}

impl ExportData {
    /// Create new export data from database
    pub fn from_db(db: &ClipboardDb) -> Result<Self, String> {
        // Get all non-sensitive entries
        let entries =
            db.get_entries(CategoryFilter::All, None, 10000, 0).map_err(|e| e.to_string())?;

        let export_entries: Vec<ExportEntry> = entries
            .into_iter()
            .filter(|e| !e.is_sensitive) // Skip sensitive entries
            .map(|e| ExportEntry::from_entry(&e))
            .collect();

        let tags = db.get_all_tags().map_err(|e| e.to_string())?;
        let export_tags: Vec<ExportTag> = tags.into_iter().map(ExportTag::from_tag).collect();

        Ok(Self {
            version: EXPORT_VERSION.to_string(),
            exported_at: Utc::now().to_rfc3339(),
            entry_count: export_entries.len(),
            entries: export_entries,
            tags: export_tags,
        })
    }

    /// Export to JSON string
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string_pretty(self).map_err(|e| e.to_string())
    }

    /// Export to file
    pub fn to_file(&self, path: &Path) -> Result<(), String> {
        let json = self.to_json()?;
        fs::write(path, json).map_err(|e| e.to_string())
    }

    /// Import from JSON string
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| e.to_string())
    }

    /// Import from file
    pub fn from_file(path: &Path) -> Result<Self, String> {
        let json = fs::read_to_string(path).map_err(|e| e.to_string())?;
        Self::from_json(&json)
    }

    /// Import entries into database
    ///
    /// If `replace` is true, clears existing entries first.
    /// Otherwise, merges with existing entries (skipping duplicates by hash).
    pub fn import_to_db(&self, db: &ClipboardDb, replace: bool) -> Result<usize, String> {
        if replace {
            db.clear_all().map_err(|e| e.to_string())?;
        }

        // Import tags first
        for tag in &self.tags {
            // Try to create, ignore if already exists
            let _ = db.create_tag(&tag.name, &tag.color);
        }

        let mut imported = 0;
        for export_entry in &self.entries {
            if let Ok(entry) = export_entry.to_entry() {
                if db.insert_entry(&entry).is_ok() {
                    imported += 1;
                }
            }
        }

        Ok(imported)
    }
}

impl ExportEntry {
    /// Create export entry from clipboard entry
    fn from_entry(entry: &ClipboardEntry) -> Self {
        let (text, image_base64, image_width, image_height, files) = match &entry.content {
            ClipboardContent::Text(t) => (Some(t.clone()), None, None, None, None),
            ClipboardContent::Image { data, width, height } => {
                use base64::{Engine as _, engine::general_purpose::STANDARD};
                let encoded = STANDARD.encode(data);
                (None, Some(encoded), Some(*width), Some(*height), None)
            }
            ClipboardContent::Files(f) => (None, None, None, None, Some(f.clone())),
        };

        Self {
            id: entry.id.clone(),
            content_type: entry.content_type.as_str().to_string(),
            text,
            image_base64,
            image_width,
            image_height,
            files,
            preview: entry.preview_text.clone(),
            is_favorite: entry.is_favorite,
            is_pinned: entry.is_pinned,
            created_at: entry.created_at.clone(),
            tag_ids: entry.tags.iter().map(|t| t.id.clone()).collect(),
        }
    }

    /// Convert export entry back to clipboard entry
    fn to_entry(&self) -> Result<ClipboardEntry, String> {
        let content = match self.content_type.as_str() {
            "text" => {
                let text = self.text.clone().ok_or("Missing text content")?;
                ClipboardContent::Text(text)
            }
            "image" => {
                use base64::{Engine as _, engine::general_purpose::STANDARD};
                let encoded = self.image_base64.as_ref().ok_or("Missing image data")?;
                let data = STANDARD.decode(encoded).map_err(|e| e.to_string())?;
                let width = self.image_width.unwrap_or(0);
                let height = self.image_height.unwrap_or(0);
                ClipboardContent::Image { data, width, height }
            }
            "files" => {
                let files = self.files.clone().ok_or("Missing files list")?;
                ClipboardContent::Files(files)
            }
            _ => return Err(format!("Unknown content type: {}", self.content_type)),
        };

        // Compute hash from content
        let hash = match &content {
            ClipboardContent::Text(t) => compute_simple_hash(t.as_bytes()),
            ClipboardContent::Image { data, .. } => compute_simple_hash(data),
            ClipboardContent::Files(f) => compute_simple_hash(f.join(",").as_bytes()),
        };

        let mut entry =
            ClipboardEntry::new(self.id.clone(), content, hash, self.created_at.clone());
        entry.is_favorite = self.is_favorite;
        entry.is_pinned = self.is_pinned;
        entry.preview_text = self.preview.clone();

        Ok(entry)
    }
}

impl ExportTag {
    fn from_tag(tag: Tag) -> Self {
        Self { id: tag.id, name: tag.name, color: tag.color }
    }
}

/// Simple hash for import (doesn't need to be SHA-256)
fn compute_simple_hash(data: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    result.iter().map(|b| format!("{:02x}", b)).collect()
}

// Re-export base64 as it's needed for image encoding
mod base64 {
    pub mod engine {
        pub mod general_purpose {
            pub struct Standard;
            pub const STANDARD: Standard = Standard;
        }
    }

    pub trait Engine {
        fn encode(&self, data: &[u8]) -> String;
        fn decode(&self, input: &str) -> Result<Vec<u8>, DecodeError>;
    }

    #[derive(Debug)]
    pub struct DecodeError;

    impl std::fmt::Display for DecodeError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "base64 decode error")
        }
    }

    impl Engine for engine::general_purpose::Standard {
        fn encode(&self, data: &[u8]) -> String {
            const ALPHABET: &[u8] =
                b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
            let mut result = String::new();
            let mut i = 0;

            while i < data.len() {
                let b0 = data[i] as usize;
                let b1 = data.get(i + 1).map(|&b| b as usize).unwrap_or(0);
                let b2 = data.get(i + 2).map(|&b| b as usize).unwrap_or(0);

                result.push(ALPHABET[b0 >> 2] as char);
                result.push(ALPHABET[((b0 & 0x03) << 4) | (b1 >> 4)] as char);

                if i + 1 < data.len() {
                    result.push(ALPHABET[((b1 & 0x0f) << 2) | (b2 >> 6)] as char);
                } else {
                    result.push('=');
                }

                if i + 2 < data.len() {
                    result.push(ALPHABET[b2 & 0x3f] as char);
                } else {
                    result.push('=');
                }

                i += 3;
            }

            result
        }

        fn decode(&self, input: &str) -> Result<Vec<u8>, DecodeError> {
            const DECODE_TABLE: [i8; 128] = [
                -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
                -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
                -1, 62, -1, -1, -1, 63, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, -1, -1, -1, -1, -1,
                -1, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
                21, 22, 23, 24, 25, -1, -1, -1, -1, -1, -1, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35,
                36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, -1, -1, -1, -1, -1,
            ];

            let input = input.trim_end_matches('=');
            let mut result = Vec::new();
            let chars: Vec<char> = input.chars().collect();
            let mut i = 0;

            while i < chars.len() {
                let mut buf = [0u8; 4];
                let mut count = 0;

                for j in 0..4 {
                    if i + j < chars.len() {
                        let c = chars[i + j] as usize;
                        if c >= 128 {
                            return Err(DecodeError);
                        }
                        let val = DECODE_TABLE[c];
                        if val < 0 {
                            return Err(DecodeError);
                        }
                        buf[j] = val as u8;
                        count += 1;
                    }
                }

                if count >= 2 {
                    result.push((buf[0] << 2) | (buf[1] >> 4));
                }
                if count >= 3 {
                    result.push((buf[1] << 4) | (buf[2] >> 2));
                }
                if count >= 4 {
                    result.push((buf[2] << 6) | buf[3]);
                }

                i += 4;
            }

            Ok(result)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64_roundtrip() {
        use base64::{Engine as _, engine::general_purpose::STANDARD};

        let original = b"Hello, World!";
        let encoded = STANDARD.encode(original);
        let decoded = STANDARD.decode(&encoded).unwrap();
        assert_eq!(original.as_slice(), decoded.as_slice());
    }

    #[test]
    fn test_base64_encode() {
        use base64::{Engine as _, engine::general_purpose::STANDARD};

        assert_eq!(STANDARD.encode(b""), "");
        assert_eq!(STANDARD.encode(b"f"), "Zg==");
        assert_eq!(STANDARD.encode(b"fo"), "Zm8=");
        assert_eq!(STANDARD.encode(b"foo"), "Zm9v");
        assert_eq!(STANDARD.encode(b"foob"), "Zm9vYg==");
        assert_eq!(STANDARD.encode(b"fooba"), "Zm9vYmE=");
        assert_eq!(STANDARD.encode(b"foobar"), "Zm9vYmFy");
    }

    #[test]
    fn test_export_entry_text() {
        let entry = ClipboardEntry::new(
            "test-id",
            ClipboardContent::Text("Hello".to_string()),
            "hash123",
            "2025-01-01T00:00:00Z",
        );

        let export = ExportEntry::from_entry(&entry);
        assert_eq!(export.content_type, "text");
        assert_eq!(export.text, Some("Hello".to_string()));
        assert!(export.image_base64.is_none());

        let restored = export.to_entry().unwrap();
        if let ClipboardContent::Text(text) = &restored.content {
            assert_eq!(text, "Hello");
        } else {
            panic!("Expected text content");
        }
    }
}
