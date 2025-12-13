use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// 剪贴板内容
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClipboardContent {
    Text(String),
    Image(Vec<u8>),
    Files(Vec<PathBuf>),
}

impl ClipboardContent {
    pub fn text(s: impl Into<String>) -> Self {
        Self::Text(s.into())
    }

    pub fn image(data: Vec<u8>) -> Self {
        Self::Image(data)
    }

    pub fn files(paths: Vec<PathBuf>) -> Self {
        Self::Files(paths)
    }

    pub fn is_text(&self) -> bool {
        matches!(self, Self::Text(_))
    }

    pub fn is_image(&self) -> bool {
        matches!(self, Self::Image(_))
    }

    pub fn is_files(&self) -> bool {
        matches!(self, Self::Files(_))
    }

    pub fn as_text(&self) -> Option<&String> {
        if let Self::Text(s) = self { Some(s) } else { None }
    }

    pub fn as_image(&self) -> Option<&Vec<u8>> {
        if let Self::Image(data) = self { Some(data) } else { None }
    }

    pub fn as_files(&self) -> Option<&Vec<PathBuf>> {
        if let Self::Files(paths) = self { Some(paths) } else { None }
    }
}

/// 剪贴板操作
#[derive(Debug, Clone)]
pub struct ClipboardOperation {
    pub content: ClipboardContent,
    pub format: ClipboardFormat,
}

impl ClipboardOperation {
    pub fn new(content: ClipboardContent, format: ClipboardFormat) -> Self {
        Self { content, format }
    }
}

/// 剪贴板格式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClipboardFormat {
    Text,
    Html,
    Rtf,
    Image,
    Files,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clipboard_content_text() {
        let content = ClipboardContent::text("Hello, World!");
        assert!(content.is_text());
        assert!(!content.is_image());
        assert_eq!(content.as_text().unwrap(), "Hello, World!");
    }

    #[test]
    fn test_clipboard_content_image() {
        let data = vec![1, 2, 3, 4, 5];
        let content = ClipboardContent::image(data.clone());
        assert!(content.is_image());
        assert!(!content.is_text());
        assert_eq!(content.as_image().unwrap(), &data);
    }

    #[test]
    fn test_clipboard_content_files() {
        let paths = vec![PathBuf::from("/test/file1.txt"), PathBuf::from("/test/file2.txt")];
        let content = ClipboardContent::files(paths.clone());
        assert!(content.is_files());
        assert!(!content.is_text());
        assert_eq!(content.as_files().unwrap(), &paths);
    }
}
