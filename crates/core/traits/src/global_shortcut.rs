use async_trait::async_trait;
use aumate_core_shared::InfrastructureError;

/// 全局快捷键管理 Port
///
/// 负责全局快捷键的注册、注销和可用性检查
///
/// **实现者**:
/// - `GlobalShortcutAdapter` (infrastructure/adapters/global_shortcut.rs)
#[async_trait]
pub trait GlobalShortcutPort: Send + Sync {
    /// 注册全局快捷键
    ///
    /// # 参数
    /// * `shortcut` - 快捷键字符串 (e.g., "Ctrl+4", "F3")
    ///
    /// # 返回
    /// * `Ok(())` - 注册成功
    /// * `Err(InfrastructureError)` - 注册失败（可能已被占用）
    async fn register(&self, shortcut: &str) -> Result<(), InfrastructureError>;

    /// 注销全局快捷键
    ///
    /// # 参数
    /// * `shortcut` - 快捷键字符串
    ///
    /// # 返回
    /// * `Ok(())` - 注销成功
    /// * `Err(InfrastructureError)` - 注销失败
    async fn unregister(&self, shortcut: &str) -> Result<(), InfrastructureError>;

    /// 检查快捷键是否可用（未被占用）
    ///
    /// # 参数
    /// * `shortcut` - 快捷键字符串
    ///
    /// # 返回
    /// * `Ok(true)` - 可用
    /// * `Ok(false)` - 不可用（已被占用）
    /// * `Err(InfrastructureError)` - 检查失败（如格式错误）
    async fn is_available(&self, shortcut: &str) -> Result<bool, InfrastructureError>;
}



