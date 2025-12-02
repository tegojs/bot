//! Clipboard entry types and data structures

use std::fmt;

/// Type of clipboard content
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContentType {
    /// Plain text content
    Text,
    /// Image content (stored as PNG)
    Image,
    /// File paths
    Files,
}

impl ContentType {
    /// Convert to string for database storage
    pub fn as_str(&self) -> &'static str {
        match self {
            ContentType::Text => "text",
            ContentType::Image => "image",
            ContentType::Files => "files",
        }
    }

    /// Parse from database string
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "text" => Some(ContentType::Text),
            "image" => Some(ContentType::Image),
            "files" => Some(ContentType::Files),
            _ => None,
        }
    }
}

impl fmt::Display for ContentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Clipboard content variants
#[derive(Debug, Clone)]
pub enum ClipboardContent {
    /// Text content
    Text(String),
    /// Image content with dimensions
    Image {
        /// PNG-encoded image data
        data: Vec<u8>,
        /// Image width in pixels
        width: u32,
        /// Image height in pixels
        height: u32,
    },
    /// List of file paths
    Files(Vec<String>),
}

impl ClipboardContent {
    /// Get the content type
    pub fn content_type(&self) -> ContentType {
        match self {
            ClipboardContent::Text(_) => ContentType::Text,
            ClipboardContent::Image { .. } => ContentType::Image,
            ClipboardContent::Files(_) => ContentType::Files,
        }
    }

    /// Generate a preview string (first 200 chars for text, dimensions for image)
    pub fn preview(&self) -> String {
        match self {
            ClipboardContent::Text(text) => {
                let preview: String = text.chars().take(200).collect();
                if text.len() > 200 { format!("{}...", preview) } else { preview }
            }
            ClipboardContent::Image { width, height, .. } => {
                format!("Image {}x{}", width, height)
            }
            ClipboardContent::Files(files) => {
                if files.len() == 1 {
                    files[0].clone()
                } else {
                    format!("{} files", files.len())
                }
            }
        }
    }
}

/// Type of sensitive data detected
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SensitiveDataType {
    /// Password or credential
    Password,
    /// API key or token
    ApiKey,
    /// Private key (SSH, PGP, etc.)
    PrivateKey,
    /// Credit card number
    CreditCard,
}

impl SensitiveDataType {
    /// Convert to string for database storage
    pub fn as_str(&self) -> &'static str {
        match self {
            SensitiveDataType::Password => "password",
            SensitiveDataType::ApiKey => "api_key",
            SensitiveDataType::PrivateKey => "private_key",
            SensitiveDataType::CreditCard => "credit_card",
        }
    }

    /// Parse from database string
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "password" => Some(SensitiveDataType::Password),
            "api_key" => Some(SensitiveDataType::ApiKey),
            "private_key" => Some(SensitiveDataType::PrivateKey),
            "credit_card" => Some(SensitiveDataType::CreditCard),
            _ => None,
        }
    }
}

impl fmt::Display for SensitiveDataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// A tag for categorizing clipboard entries
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tag {
    /// Unique identifier
    pub id: String,
    /// Tag name
    pub name: String,
    /// Tag color (hex format, e.g., "#FF5733")
    pub color: String,
}

impl Tag {
    /// Create a new tag
    pub fn new(id: impl Into<String>, name: impl Into<String>, color: impl Into<String>) -> Self {
        Self { id: id.into(), name: name.into(), color: color.into() }
    }
}

/// A clipboard history entry
#[derive(Debug, Clone)]
pub struct ClipboardEntry {
    /// Unique identifier (UUID)
    pub id: String,
    /// Type of content
    pub content_type: ContentType,
    /// The actual content
    pub content: ClipboardContent,
    /// Preview text for display
    pub preview_text: String,
    /// SHA-256 hash of content for deduplication
    pub hash: String,
    /// Whether this entry contains sensitive data
    pub is_sensitive: bool,
    /// Type of sensitive data if detected
    pub sensitive_type: Option<SensitiveDataType>,
    /// Whether this entry is marked as favorite
    pub is_favorite: bool,
    /// Whether this entry is pinned to top
    pub is_pinned: bool,
    /// ISO 8601 timestamp of creation
    pub created_at: String,
    /// ISO 8601 timestamp of last access
    pub accessed_at: String,
    /// Number of times this entry was accessed
    pub access_count: u32,
    /// Source application name (if available)
    pub source_app: Option<String>,
    /// Associated tags
    pub tags: Vec<Tag>,
}

impl ClipboardEntry {
    /// Create a new clipboard entry
    pub fn new(
        id: impl Into<String>,
        content: ClipboardContent,
        hash: impl Into<String>,
        created_at: impl Into<String>,
    ) -> Self {
        let preview_text = content.preview();
        let content_type = content.content_type();
        let created = created_at.into();

        Self {
            id: id.into(),
            content_type,
            content,
            preview_text,
            hash: hash.into(),
            is_sensitive: false,
            sensitive_type: None,
            is_favorite: false,
            is_pinned: false,
            created_at: created.clone(),
            accessed_at: created,
            access_count: 1,
            source_app: None,
            tags: Vec::new(),
        }
    }
}

/// Filter categories for browsing clipboard history
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CategoryFilter {
    /// Show all entries
    #[default]
    All,
    /// Show only text entries
    Text,
    /// Show only image entries
    Images,
    /// Show only file entries
    Files,
    /// Show only favorite entries
    Favorites,
    /// Show only sensitive entries
    Sensitive,
}

impl CategoryFilter {
    /// Get display name
    pub fn display_name(&self) -> &'static str {
        match self {
            CategoryFilter::All => "All",
            CategoryFilter::Text => "Text",
            CategoryFilter::Images => "Images",
            CategoryFilter::Files => "Files",
            CategoryFilter::Favorites => "Favorites",
            CategoryFilter::Sensitive => "Sensitive",
        }
    }

    /// Get all filter options
    pub fn all_options() -> &'static [CategoryFilter] {
        &[
            CategoryFilter::All,
            CategoryFilter::Text,
            CategoryFilter::Images,
            CategoryFilter::Files,
            CategoryFilter::Favorites,
            CategoryFilter::Sensitive,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_content_type_conversion() {
        assert_eq!(ContentType::Text.as_str(), "text");
        assert_eq!(ContentType::parse("text"), Some(ContentType::Text));
        assert_eq!(ContentType::parse("unknown"), None);
    }

    #[test]
    fn test_clipboard_content_preview() {
        let text = ClipboardContent::Text("Hello, World!".to_string());
        assert_eq!(text.preview(), "Hello, World!");

        let long_text = ClipboardContent::Text("a".repeat(250));
        assert!(long_text.preview().ends_with("..."));
        assert!(long_text.preview().len() < 250);

        let image = ClipboardContent::Image { data: vec![], width: 1920, height: 1080 };
        assert_eq!(image.preview(), "Image 1920x1080");

        let files = ClipboardContent::Files(vec![
            "/path/to/file1.txt".to_string(),
            "/path/to/file2.txt".to_string(),
        ]);
        assert_eq!(files.preview(), "2 files");
    }

    #[test]
    fn test_sensitive_data_type() {
        assert_eq!(SensitiveDataType::Password.as_str(), "password");
        assert_eq!(SensitiveDataType::parse("api_key"), Some(SensitiveDataType::ApiKey));
    }
}
