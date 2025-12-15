use aumate_core_shared::UseCaseError;
use aumate_core_traits::GlobalShortcutPort;
use std::sync::Arc;

/// 注册全局快捷键 Use Case
pub struct RegisterGlobalShortcutUseCase {
    global_shortcut: Arc<dyn GlobalShortcutPort>,
}

impl RegisterGlobalShortcutUseCase {
    pub fn new(global_shortcut: Arc<dyn GlobalShortcutPort>) -> Self {
        Self { global_shortcut }
    }

    /// 注册全局快捷键
    ///
    /// # 参数
    /// * `shortcut` - 快捷键字符串 (e.g., "Ctrl+4", "F3")
    ///
    /// # 返回
    /// * `Ok(())` - 注册成功
    /// * `Err(UseCaseError)` - 注册失败
    pub async fn execute(&self, shortcut: String) -> Result<(), UseCaseError> {
        log::info!("[RegisterGlobalShortcutUseCase] Registering shortcut: {}", shortcut);

        self.global_shortcut
            .register(&shortcut)
            .await
            .map_err(|e| {
                log::error!("[RegisterGlobalShortcutUseCase] Failed to register shortcut '{}': {}", shortcut, e);
                UseCaseError::from(e)
            })?;

        log::info!("[RegisterGlobalShortcutUseCase] Successfully registered shortcut: {}", shortcut);
        Ok(())
    }
}

/// 注销全局快捷键 Use Case
pub struct UnregisterGlobalShortcutUseCase {
    global_shortcut: Arc<dyn GlobalShortcutPort>,
}

impl UnregisterGlobalShortcutUseCase {
    pub fn new(global_shortcut: Arc<dyn GlobalShortcutPort>) -> Self {
        Self { global_shortcut }
    }

    /// 注销全局快捷键
    ///
    /// # 参数
    /// * `shortcut` - 快捷键字符串
    ///
    /// # 返回
    /// * `Ok(())` - 注销成功
    /// * `Err(UseCaseError)` - 注销失败
    pub async fn execute(&self, shortcut: String) -> Result<(), UseCaseError> {
        log::info!("[UnregisterGlobalShortcutUseCase] Unregistering shortcut: {}", shortcut);

        self.global_shortcut
            .unregister(&shortcut)
            .await
            .map_err(|e| {
                log::error!("[UnregisterGlobalShortcutUseCase] Failed to unregister shortcut '{}': {}", shortcut, e);
                UseCaseError::from(e)
            })?;

        log::info!("[UnregisterGlobalShortcutUseCase] Successfully unregistered shortcut: {}", shortcut);
        Ok(())
    }
}

/// 检查全局快捷键可用性 Use Case
pub struct CheckGlobalShortcutAvailabilityUseCase {
    global_shortcut: Arc<dyn GlobalShortcutPort>,
}

impl CheckGlobalShortcutAvailabilityUseCase {
    pub fn new(global_shortcut: Arc<dyn GlobalShortcutPort>) -> Self {
        Self { global_shortcut }
    }

    /// 检查快捷键是否可用
    ///
    /// # 参数
    /// * `shortcut` - 快捷键字符串
    ///
    /// # 返回
    /// * `Ok(true)` - 可用
    /// * `Ok(false)` - 不可用（已被占用）
    /// * `Err(UseCaseError)` - 检查失败
    pub async fn execute(&self, shortcut: String) -> Result<bool, UseCaseError> {
        log::info!("[CheckGlobalShortcutAvailabilityUseCase] Checking availability of shortcut: {}", shortcut);

        let available = self
            .global_shortcut
            .is_available(&shortcut)
            .await
            .map_err(|e| {
                log::error!("[CheckGlobalShortcutAvailabilityUseCase] Failed to check shortcut '{}': {}", shortcut, e);
                UseCaseError::from(e)
            })?;

        log::info!("[CheckGlobalShortcutAvailabilityUseCase] Shortcut '{}' is {}", shortcut, if available { "available" } else { "not available" });
        Ok(available)
    }
}

