use async_trait::async_trait;
use aumate_core_shared::InfrastructureError;
use std::path::{Path, PathBuf};
use std::time::Duration;

/// 文件系统 Port
///
/// 负责文件系统操作
///
/// **实现者**:
/// - `FileSystemAdapter`
#[async_trait]
pub trait FileSystemPort: Send + Sync {
    /// 读取文件
    async fn read(&self, path: &Path) -> Result<Vec<u8>, InfrastructureError>;

    /// 写入文件
    async fn write(&self, path: &Path, content: &[u8]) -> Result<(), InfrastructureError>;

    /// 删除文件
    async fn delete(&self, path: &Path) -> Result<(), InfrastructureError>;

    /// 复制文件
    async fn copy(&self, from: &Path, to: &Path) -> Result<(), InfrastructureError>;

    /// 移动文件
    async fn move_file(&self, from: &Path, to: &Path) -> Result<(), InfrastructureError>;

    /// 检查文件是否存在
    async fn exists(&self, path: &Path) -> bool;

    /// 创建目录
    async fn create_dir(&self, path: &Path) -> Result<(), InfrastructureError>;

    /// 删除目录
    async fn remove_dir(&self, path: &Path) -> Result<(), InfrastructureError>;

    /// 获取应用配置目录
    fn get_app_config_dir(&self) -> Result<PathBuf, InfrastructureError>;
}

/// 缓存 Port
///
/// 负责缓存操作
///
/// **实现者**:
/// - `MemoryCacheAdapter<T>`
#[async_trait]
pub trait CachePort<T>: Send + Sync
where
    T: Clone + Send + Sync,
{
    /// 获取缓存
    async fn get(&self, key: &str) -> Option<T>;

    /// 设置缓存
    async fn set(&mut self, key: String, value: T, ttl: Option<Duration>);

    /// 删除缓存
    async fn remove(&mut self, key: &str);

    /// 清空所有缓存
    async fn clear(&mut self);
}
