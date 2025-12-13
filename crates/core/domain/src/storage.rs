use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::Duration;

/// 文件操作类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FileOperationType {
    Read,
    Write { content: Vec<u8> },
    Delete,
    Copy { to: PathBuf },
    Move { to: PathBuf },
}

/// 文件操作
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileOperation {
    pub operation_type: FileOperationType,
    pub path: PathBuf,
}

impl FileOperation {
    pub fn read(path: PathBuf) -> Self {
        Self { operation_type: FileOperationType::Read, path }
    }

    pub fn write(path: PathBuf, content: Vec<u8>) -> Self {
        Self { operation_type: FileOperationType::Write { content }, path }
    }

    pub fn delete(path: PathBuf) -> Self {
        Self { operation_type: FileOperationType::Delete, path }
    }

    pub fn copy(path: PathBuf, to: PathBuf) -> Self {
        Self { operation_type: FileOperationType::Copy { to }, path }
    }

    pub fn move_to(path: PathBuf, to: PathBuf) -> Self {
        Self { operation_type: FileOperationType::Move { to }, path }
    }
}

/// 缓存条目
#[derive(Debug, Clone)]
pub struct CacheEntry<T> {
    pub key: String,
    pub value: T,
    pub expires_at: Option<std::time::SystemTime>,
}

impl<T> CacheEntry<T> {
    pub fn new(key: String, value: T) -> Self {
        Self { key, value, expires_at: None }
    }

    pub fn with_ttl(key: String, value: T, ttl: Duration) -> Self {
        let expires_at = std::time::SystemTime::now() + ttl;
        Self { key, value, expires_at: Some(expires_at) }
    }

    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            std::time::SystemTime::now() > expires_at
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_operation_creation() {
        let op = FileOperation::read(PathBuf::from("/test/file.txt"));
        assert!(matches!(op.operation_type, FileOperationType::Read));

        let op = FileOperation::write(PathBuf::from("/test/file.txt"), vec![1, 2, 3]);
        assert!(matches!(op.operation_type, FileOperationType::Write { .. }));

        let op = FileOperation::delete(PathBuf::from("/test/file.txt"));
        assert!(matches!(op.operation_type, FileOperationType::Delete));
    }

    #[test]
    fn test_cache_entry_expiration() {
        let entry = CacheEntry::new("key".to_string(), "value".to_string());
        assert!(!entry.is_expired());

        let entry =
            CacheEntry::with_ttl("key".to_string(), "value".to_string(), Duration::from_millis(1));
        std::thread::sleep(Duration::from_millis(10));
        assert!(entry.is_expired());
    }
}
