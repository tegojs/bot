// 窗口毛玻璃效果相关 Use Cases
use aumate_core_shared::{UseCaseError, WindowId};
use aumate_core_traits::{VibrancyEffect, WindowVibrancyPort};
use std::sync::Arc;

/// 设置窗口毛玻璃效果用例
///
/// 提供启用/禁用窗口毛玻璃效果的功能
pub struct SetWindowVibrancyUseCase {
    window_vibrancy: Arc<dyn WindowVibrancyPort>,
}

impl SetWindowVibrancyUseCase {
    pub fn new(window_vibrancy: Arc<dyn WindowVibrancyPort>) -> Self {
        Self { window_vibrancy }
    }

    /// 执行设置窗口毛玻璃效果
    ///
    /// # Arguments
    /// * `window_id` - 窗口标识
    /// * `enabled` - 是否启用毛玻璃效果
    /// * `effect` - 毛玻璃效果类型（仅在 enabled 为 true 时有效）
    pub async fn execute(
        &self,
        window_id: WindowId,
        enabled: bool,
        effect: Option<VibrancyEffect>,
    ) -> Result<(), UseCaseError> {
        log::info!(
            "[SetWindowVibrancyUseCase] Executing set vibrancy for {:?}, enabled={}",
            window_id,
            enabled
        );

        if enabled {
            // 默认使用 Acrylic (Windows) 或 HudWindow (macOS)
            let effect = effect.unwrap_or_else(|| {
                #[cfg(target_os = "windows")]
                {
                    VibrancyEffect::Acrylic
                }
                #[cfg(target_os = "macos")]
                {
                    VibrancyEffect::HudWindow
                }
                #[cfg(not(any(target_os = "windows", target_os = "macos")))]
                {
                    VibrancyEffect::Acrylic
                }
            });

            self.window_vibrancy
                .enable_vibrancy(&window_id, effect)
                .await
                .map_err(|e| e.into())
        } else {
            self.window_vibrancy
                .disable_vibrancy(&window_id)
                .await
                .map_err(|e| e.into())
        }
    }
}

/// 检查窗口毛玻璃效果状态用例
pub struct GetWindowVibrancyStateUseCase {
    window_vibrancy: Arc<dyn WindowVibrancyPort>,
}

impl GetWindowVibrancyStateUseCase {
    pub fn new(window_vibrancy: Arc<dyn WindowVibrancyPort>) -> Self {
        Self { window_vibrancy }
    }

    /// 执行检查窗口毛玻璃效果状态
    pub async fn execute(&self, window_id: WindowId) -> Result<bool, UseCaseError> {
        log::info!(
            "[GetWindowVibrancyStateUseCase] Checking vibrancy state for {:?}",
            window_id
        );

        self.window_vibrancy
            .is_vibrancy_enabled(&window_id)
            .await
            .map_err(|e| e.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use aumate_core_shared::InfrastructureError;

    struct MockWindowVibrancyPort {
        enabled: std::sync::atomic::AtomicBool,
    }

    impl MockWindowVibrancyPort {
        fn new() -> Self {
            Self {
                enabled: std::sync::atomic::AtomicBool::new(false),
            }
        }
    }

    #[async_trait]
    impl WindowVibrancyPort for MockWindowVibrancyPort {
        async fn enable_vibrancy(
            &self,
            _window_id: &WindowId,
            _effect: VibrancyEffect,
        ) -> Result<(), InfrastructureError> {
            self.enabled
                .store(true, std::sync::atomic::Ordering::SeqCst);
            Ok(())
        }

        async fn disable_vibrancy(
            &self,
            _window_id: &WindowId,
        ) -> Result<(), InfrastructureError> {
            self.enabled
                .store(false, std::sync::atomic::Ordering::SeqCst);
            Ok(())
        }

        async fn is_vibrancy_enabled(
            &self,
            _window_id: &WindowId,
        ) -> Result<bool, InfrastructureError> {
            Ok(self.enabled.load(std::sync::atomic::Ordering::SeqCst))
        }
    }

    #[tokio::test]
    async fn test_set_window_vibrancy_enable() {
        let port = Arc::new(MockWindowVibrancyPort::new());
        let use_case = SetWindowVibrancyUseCase::new(port.clone());

        let window_id = WindowId::new("test".to_string());
        let result = use_case.execute(window_id, true, None).await;

        assert!(result.is_ok());
        assert!(port.enabled.load(std::sync::atomic::Ordering::SeqCst));
    }

    #[tokio::test]
    async fn test_set_window_vibrancy_disable() {
        let port = Arc::new(MockWindowVibrancyPort::new());
        port.enabled
            .store(true, std::sync::atomic::Ordering::SeqCst);

        let use_case = SetWindowVibrancyUseCase::new(port.clone());
        let window_id = WindowId::new("test".to_string());
        let result = use_case.execute(window_id, false, None).await;

        assert!(result.is_ok());
        assert!(!port.enabled.load(std::sync::atomic::Ordering::SeqCst));
    }
}
