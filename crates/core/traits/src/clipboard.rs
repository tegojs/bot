use async_trait::async_trait;
use aumate_core_shared::InfrastructureError;
use std::path::PathBuf;

/// 剪贴板内容
#[derive(Debug, Clone)]
pub enum ClipboardContent {
    Text(String),
    Image(Vec<u8>),
    Files(Vec<PathBuf>),
}

/// 剪贴板内容类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClipboardType {
    Text,
    Image,
    Files,
}

/// 剪贴板 Port
///
/// 负责剪贴板操作
///
/// **实现者**:
/// - `WindowsClipboardAdapter`
/// - `MacOSClipboardAdapter`
/// - `LinuxClipboardAdapter`
#[async_trait]
pub trait ClipboardPort: Send + Sync {
    /// 读取剪贴板内容
    async fn read(&self) -> Result<ClipboardContent, InfrastructureError>;

    /// 写入剪贴板
    async fn write(&self, content: ClipboardContent) -> Result<(), InfrastructureError>;

    /// 清空剪贴板
    async fn clear(&self) -> Result<(), InfrastructureError>;

    /// 检查剪贴板内容类型
    async fn get_available_types(&self) -> Result<Vec<ClipboardType>, InfrastructureError>;
}
