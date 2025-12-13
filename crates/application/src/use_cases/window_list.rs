// 窗口列表相关 Use Cases
use aumate_core_shared::UseCaseError;
use aumate_core_traits::{WindowListPort, window::WindowInfo};
use std::sync::Arc;

/// 获取窗口列表用例
///
/// 提供获取系统所有可见窗口列表的功能
pub struct GetWindowElementsUseCase {
    window_list: Arc<dyn WindowListPort>,
}

impl GetWindowElementsUseCase {
    pub fn new(window_list: Arc<dyn WindowListPort>) -> Self {
        Self { window_list }
    }

    /// 执行获取窗口列表
    ///
    /// 返回所有可见窗口的列表
    pub async fn execute(&self) -> Result<Vec<WindowInfo>, UseCaseError> {
        log::info!("[GetWindowElementsUseCase] Executing get window elements");

        self.window_list
            .get_window_list()
            .await
            .map_err(|e| e.into())
    }
}

/// 获取当前活动窗口用例
pub struct GetActiveWindowUseCase {
    window_list: Arc<dyn WindowListPort>,
}

impl GetActiveWindowUseCase {
    pub fn new(window_list: Arc<dyn WindowListPort>) -> Self {
        Self { window_list }
    }

    /// 执行获取当前活动窗口
    pub async fn execute(&self) -> Result<Option<WindowInfo>, UseCaseError> {
        log::info!("[GetActiveWindowUseCase] Executing get active window");

        self.window_list
            .get_active_window()
            .await
            .map_err(|e| e.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use aumate_core_shared::InfrastructureError;

    struct MockWindowListPort;

    #[async_trait]
    impl WindowListPort for MockWindowListPort {
        async fn get_window_list(&self) -> Result<Vec<WindowInfo>, InfrastructureError> {
            Ok(vec![])
        }

        async fn get_active_window(&self) -> Result<Option<WindowInfo>, InfrastructureError> {
            Ok(None)
        }
    }

    #[tokio::test]
    async fn test_get_window_elements_use_case() {
        let port = Arc::new(MockWindowListPort);
        let use_case = GetWindowElementsUseCase::new(port);
        let result = use_case.execute().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_active_window_use_case() {
        let port = Arc::new(MockWindowListPort);
        let use_case = GetActiveWindowUseCase::new(port);
        let result = use_case.execute().await;
        assert!(result.is_ok());
    }
}
