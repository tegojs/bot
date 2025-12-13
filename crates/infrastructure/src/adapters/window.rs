// 窗口管理适配器
use async_trait::async_trait;
use aumate_core_shared::{InfrastructureError, WindowId};
use aumate_core_traits::WindowManagementPort;
use aumate_core_traits::window::{
    DragOperation, DrawWindowStyle, ResizeOperation, Window, WindowConfig,
};

/// 窗口管理适配器
///
/// 封装 Tauri 窗口管理功能
pub struct WindowManagementAdapter {
    // 可以存储 Tauri App Handle 或其他状态
}

impl WindowManagementAdapter {
    pub fn new() -> Self {
        log::info!("Creating WindowManagementAdapter");
        Self {}
    }
}

impl Default for WindowManagementAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl WindowManagementPort for WindowManagementAdapter {
    async fn get_window(&self, id: WindowId) -> Result<Window, InfrastructureError> {
        log::info!("WindowManagementAdapter: getting window {}", id.as_str());

        // TODO: 实现真实的窗口查询
        Err(InfrastructureError::PlatformOperationFailed(
            "get_window not yet implemented - needs Tauri App Handle".to_string(),
        ))
    }

    async fn set_always_on_top(
        &self,
        id: WindowId,
        enable: bool,
    ) -> Result<(), InfrastructureError> {
        log::info!("WindowManagementAdapter: set_always_on_top {} = {}", id.as_str(), enable);

        Ok(())
    }

    async fn start_drag(&self, operation: DragOperation) -> Result<(), InfrastructureError> {
        log::info!(
            "WindowManagementAdapter: start_drag for window {}",
            operation.window_id.as_str()
        );

        Ok(())
    }

    async fn start_resize(&self, operation: ResizeOperation) -> Result<(), InfrastructureError> {
        log::info!(
            "WindowManagementAdapter: start_resize for window {}",
            operation.window_id.as_str()
        );

        Ok(())
    }

    async fn set_draw_style(
        &self,
        id: WindowId,
        _style: DrawWindowStyle,
    ) -> Result<(), InfrastructureError> {
        log::info!("WindowManagementAdapter: set_draw_style for window {}", id.as_str());

        #[cfg(target_os = "macos")]
        {
            Ok(())
        }

        #[cfg(not(target_os = "macos"))]
        {
            Err(InfrastructureError::PlatformNotSupported)
        }
    }

    async fn create_fixed_content_window(
        &self,
        config: WindowConfig,
    ) -> Result<WindowId, InfrastructureError> {
        log::info!("WindowManagementAdapter: creating window {}", config.title);

        // TODO: 实现真实的窗口创建
        // 暂时返回一个占位符 ID
        Ok(WindowId::new("placeholder".to_string()))
    }

    async fn close_window(&self, id: WindowId) -> Result<(), InfrastructureError> {
        log::info!("WindowManagementAdapter: closing window {}", id.as_str());

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adapter_creation() {
        let adapter = WindowManagementAdapter::new();
        drop(adapter);
    }
}
