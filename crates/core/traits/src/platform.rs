use aumate_core_shared::{InfrastructureError, Platform};

/// 平台信息 Port
///
/// 负责平台信息获取
///
/// **实现者**:
/// - `PlatformInfoAdapter`
pub trait PlatformInfoPort: Send + Sync {
    /// 获取平台类型
    fn get_platform(&self) -> Platform;

    /// 检查是否以管理员权限运行
    fn is_admin(&self) -> bool;

    /// 以管理员权限重启
    fn restart_with_admin(&self) -> Result<(), InfrastructureError>;

    /// 获取系统版本
    fn get_os_version(&self) -> String;
}
