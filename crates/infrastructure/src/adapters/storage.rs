use async_trait::async_trait;
use aumate_core_shared::InfrastructureError;
use aumate_core_traits::{CachePort, FileSystemPort};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::fs;
use tokio::sync::Mutex;

/// 文件系统适配器
///
/// 实现 `FileSystemPort` trait，提供异步文件操作
///
/// **复用代码**:
/// - `tauri-commands/file::*` - 文件操作逻辑
/// - `app-utils::save_image_to_file` - 图像保存逻辑
pub struct FileSystemAdapter {
    app_config_dir: Option<PathBuf>,
}

impl FileSystemAdapter {
    pub fn new() -> Self {
        Self { app_config_dir: None }
    }

    /// 设置应用配置目录
    pub fn with_config_dir(mut self, dir: PathBuf) -> Self {
        self.app_config_dir = Some(dir);
        self
    }
}

impl Default for FileSystemAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl FileSystemPort for FileSystemAdapter {
    /// 读取文件
    async fn read(&self, path: &Path) -> Result<Vec<u8>, InfrastructureError> {
        fs::read(path).await.map_err(|e| {
            InfrastructureError::FileOperationFailed(format!(
                "Failed to read file {:?}: {}",
                path, e
            ))
        })
    }

    /// 写入文件
    ///
    /// **复用**: `tauri-commands/file::write_file` 逻辑
    async fn write(&self, path: &Path, content: &[u8]) -> Result<(), InfrastructureError> {
        // 确保父目录存在
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent).await.map_err(|e| {
                    InfrastructureError::FileOperationFailed(format!(
                        "Failed to create directory {:?}: {}",
                        parent, e
                    ))
                })?;
            }
        }

        fs::write(path, content).await.map_err(|e| {
            InfrastructureError::FileOperationFailed(format!(
                "Failed to write file {:?}: {}",
                path, e
            ))
        })
    }

    /// 删除文件
    async fn delete(&self, path: &Path) -> Result<(), InfrastructureError> {
        fs::remove_file(path).await.map_err(|e| {
            InfrastructureError::FileOperationFailed(format!(
                "Failed to delete file {:?}: {}",
                path, e
            ))
        })
    }

    /// 复制文件
    async fn copy(&self, from: &Path, to: &Path) -> Result<(), InfrastructureError> {
        // 确保目标目录存在
        if let Some(parent) = to.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent).await.map_err(|e| {
                    InfrastructureError::FileOperationFailed(format!(
                        "Failed to create directory {:?}: {}",
                        parent, e
                    ))
                })?;
            }
        }

        fs::copy(from, to).await.map_err(|e| {
            InfrastructureError::FileOperationFailed(format!(
                "Failed to copy file {:?} to {:?}: {}",
                from, to, e
            ))
        })?;

        Ok(())
    }

    /// 移动文件
    async fn move_file(&self, from: &Path, to: &Path) -> Result<(), InfrastructureError> {
        // 确保目标目录存在
        if let Some(parent) = to.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent).await.map_err(|e| {
                    InfrastructureError::FileOperationFailed(format!(
                        "Failed to create directory {:?}: {}",
                        parent, e
                    ))
                })?;
            }
        }

        fs::rename(from, to).await.map_err(|e| {
            InfrastructureError::FileOperationFailed(format!(
                "Failed to move file {:?} to {:?}: {}",
                from, to, e
            ))
        })
    }

    /// 检查文件是否存在
    async fn exists(&self, path: &Path) -> bool {
        fs::metadata(path).await.is_ok()
    }

    /// 创建目录
    async fn create_dir(&self, path: &Path) -> Result<(), InfrastructureError> {
        fs::create_dir_all(path).await.map_err(|e| {
            InfrastructureError::FileOperationFailed(format!(
                "Failed to create directory {:?}: {}",
                path, e
            ))
        })
    }

    /// 删除目录
    async fn remove_dir(&self, path: &Path) -> Result<(), InfrastructureError> {
        fs::remove_dir_all(path).await.map_err(|e| {
            InfrastructureError::FileOperationFailed(format!(
                "Failed to remove directory {:?}: {}",
                path, e
            ))
        })
    }

    /// 获取应用配置目录
    fn get_app_config_dir(&self) -> Result<PathBuf, InfrastructureError> {
        self.app_config_dir.clone().ok_or_else(|| {
            InfrastructureError::FileOperationFailed("App config directory not set".to_string())
        })
    }
}

/// 缓存条目（内部使用）
struct CacheEntryInternal<T> {
    value: T,
    expires_at: Option<SystemTime>,
}

impl<T: Clone> CacheEntryInternal<T> {
    fn new(value: T, ttl: Option<Duration>) -> Self {
        let expires_at = ttl.map(|d| SystemTime::now() + d);
        Self { value, expires_at }
    }

    fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at { SystemTime::now() > expires_at } else { false }
    }
}

/// 内存缓存适配器
///
/// 实现 `CachePort` trait，提供内存缓存功能
///
/// **复用代码**:
/// - `app-services::FileCacheService` - 缓存逻辑（重构为泛型）
pub struct MemoryCacheAdapter<T: Clone + Send + Sync> {
    cache: Arc<Mutex<HashMap<String, CacheEntryInternal<T>>>>,
}

impl<T: Clone + Send + Sync> MemoryCacheAdapter<T> {
    pub fn new() -> Self {
        Self { cache: Arc::new(Mutex::new(HashMap::new())) }
    }

    /// 清理过期条目
    async fn cleanup_expired(&self) {
        let mut cache = self.cache.lock().await;
        cache.retain(|_, entry| !entry.is_expired());
    }
}

impl<T: Clone + Send + Sync> Default for MemoryCacheAdapter<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl<T: Clone + Send + Sync + 'static> CachePort<T> for MemoryCacheAdapter<T> {
    /// 获取缓存
    async fn get(&self, key: &str) -> Option<T> {
        let cache = self.cache.lock().await;

        if let Some(entry) = cache.get(key) {
            if !entry.is_expired() {
                return Some(entry.value.clone());
            }
        }

        None
    }

    /// 设置缓存
    async fn set(&mut self, key: String, value: T, ttl: Option<Duration>) {
        let entry = CacheEntryInternal::new(value, ttl);
        let mut cache = self.cache.lock().await;
        cache.insert(key, entry);

        // 定期清理过期条目（每 100 次插入清理一次）
        if cache.len() % 100 == 0 {
            drop(cache); // 释放锁
            self.cleanup_expired().await;
        }
    }

    /// 删除缓存
    async fn remove(&mut self, key: &str) {
        let mut cache = self.cache.lock().await;
        cache.remove(key);
    }

    /// 清空所有缓存
    async fn clear(&mut self) {
        let mut cache = self.cache.lock().await;
        cache.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_file_system_adapter_write_and_read() {
        let adapter = FileSystemAdapter::new();
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("test_aumate_write.txt");

        let content = b"Hello, Snow Shot!";

        // 写入
        let result = adapter.write(&test_file, content).await;
        assert!(result.is_ok());

        // 读取
        let read_result = adapter.read(&test_file).await;
        assert!(read_result.is_ok());
        assert_eq!(read_result.unwrap(), content);

        // 清理
        let _ = adapter.delete(&test_file).await;
    }

    #[tokio::test]
    async fn test_file_system_adapter_exists() {
        let adapter = FileSystemAdapter::new();
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("test_aumate_exists.txt");

        // 文件不存在
        assert!(!adapter.exists(&test_file).await);

        // 创建文件
        adapter.write(&test_file, b"test").await.unwrap();

        // 文件存在
        assert!(adapter.exists(&test_file).await);

        // 清理
        adapter.delete(&test_file).await.unwrap();
    }

    #[tokio::test]
    async fn test_file_system_adapter_copy() {
        let adapter = FileSystemAdapter::new();
        let temp_dir = std::env::temp_dir();
        let source = temp_dir.join("test_aumate_source.txt");
        let dest = temp_dir.join("test_aumate_dest.txt");

        // 创建源文件
        adapter.write(&source, b"copy test").await.unwrap();

        // 复制
        let result = adapter.copy(&source, &dest).await;
        assert!(result.is_ok());

        // 验证目标文件
        assert!(adapter.exists(&dest).await);
        let content = adapter.read(&dest).await.unwrap();
        assert_eq!(content, b"copy test");

        // 清理
        let _ = adapter.delete(&source).await;
        let _ = adapter.delete(&dest).await;
    }

    #[tokio::test]
    async fn test_memory_cache_basic() {
        let mut cache = MemoryCacheAdapter::<String>::new();

        // 设置缓存
        cache.set("key1".to_string(), "value1".to_string(), None).await;

        // 获取缓存
        let value = cache.get("key1").await;
        assert_eq!(value, Some("value1".to_string()));

        // 获取不存在的键
        let value = cache.get("key2").await;
        assert_eq!(value, None);
    }

    #[tokio::test]
    async fn test_memory_cache_expiration() {
        let mut cache = MemoryCacheAdapter::<String>::new();

        // 设置带 TTL 的缓存（100ms）
        cache.set("key1".to_string(), "value1".to_string(), Some(Duration::from_millis(100))).await;

        // 立即读取应该成功
        let value = cache.get("key1").await;
        assert_eq!(value, Some("value1".to_string()));

        // 等待过期
        tokio::time::sleep(Duration::from_millis(150)).await;

        // 过期后应该返回 None
        let value = cache.get("key1").await;
        assert_eq!(value, None);
    }

    #[tokio::test]
    async fn test_memory_cache_remove() {
        let mut cache = MemoryCacheAdapter::<String>::new();

        cache.set("key1".to_string(), "value1".to_string(), None).await;
        assert!(cache.get("key1").await.is_some());

        cache.remove("key1").await;
        assert!(cache.get("key1").await.is_none());
    }

    #[tokio::test]
    async fn test_memory_cache_clear() {
        let mut cache = MemoryCacheAdapter::<String>::new();

        cache.set("key1".to_string(), "value1".to_string(), None).await;
        cache.set("key2".to_string(), "value2".to_string(), None).await;

        cache.clear().await;

        assert!(cache.get("key1").await.is_none());
        assert!(cache.get("key2").await.is_none());
    }
}
