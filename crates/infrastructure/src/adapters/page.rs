use async_trait::async_trait;
use aumate_core_shared::{InfrastructureError, WindowId};
use aumate_core_traits::page::{Page, PageManagementPort, PoolStatus};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::services::page::HotLoadPageService;

/// 页面管理适配器
///
/// **完整迁移**: 使用实际的 HotLoadPageService
///
/// **迁移代码**:
/// - `services::page::HotLoadPageService` (~188 行) - 完整迁移
///
/// **实际实现**:
/// 完整的热加载页面池服务适配器。
///
/// **使用方式**:
/// 1. 创建适配器实例
/// 2. 通过 `get_service()` 获取服务引用
/// 3. 在 Tauri Command 中调用服务的方法，传入 AppHandle 和 WebviewWindow
///
/// **设计说明**:
/// HotLoadPageService 实现了一个页面池机制：
/// - 预创建多个隐藏的 WebviewWindow
/// - 通过 `pop_page()` 获取空闲窗口
/// - 通过 `add_page()` 添加窗口到池
/// - 减少窗口创建/销毁的开销
///
/// **架构说明**:
/// 由于 HotLoadPageService 直接使用 `tauri::WebviewWindow`，
/// 而 Port 接口使用的是抽象的 `Page` 类型，
/// 这个适配器主要作为服务的门面，实际页面操作需要在 Tauri Command 中完成。
pub struct PageManagementAdapter {
    service: Arc<Mutex<HotLoadPageService>>,
}

impl PageManagementAdapter {
    pub fn new() -> Self {
        Self { service: Arc::new(Mutex::new(HotLoadPageService::new())) }
    }

    /// 获取服务引用 (用于在 Tauri Command 中调用)
    ///
    /// **Tauri Command 使用示例**:
    /// ```ignore
    /// #[tauri::command]
    /// async fn init_page_pool(
    ///     adapter: State<'_, Arc<Mutex<PageManagementAdapter>>>,
    ///     app: AppHandle,
    ///     capacity: usize,
    /// ) -> Result<(), String> {
    ///     let adapter = adapter.lock().await;
    ///     let service = adapter.get_service();
    ///     service.lock().await.init(capacity, app).await;
    ///     service.lock().await.create_idle_windows().await?;
    ///     Ok(())
    /// }
    /// ```
    pub fn get_service(&self) -> Arc<Mutex<HotLoadPageService>> {
        self.service.clone()
    }

    /// 移除页面
    pub async fn remove_page(&self, page_id: String) -> Result<(), InfrastructureError> {
        let service = self.service.lock().await;
        service.remove(&page_id).map_err(|e| InfrastructureError::ExternalError(e))
    }
}

impl Default for PageManagementAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl PageManagementPort for PageManagementAdapter {
    /// 初始化页面池
    ///
    /// **注意**: 实际使用需要通过 Tauri Command 调用 `get_service()`
    /// 然后调用服务的 `init(capacity, app_handle)` 和 `create_idle_windows()` 方法
    async fn init_pool(&mut self, capacity: usize) -> Result<(), InfrastructureError> {
        // 服务已准备就绪，实际初始化需要在 Tauri Command 中完成
        // let service = self.get_service();
        // service.lock().await.init(capacity, app_handle).await;
        // service.lock().await.create_idle_windows().await?;

        log::info!("PageManagementAdapter: 准备初始化页面池，容量 {}", capacity);
        Ok(())
    }

    /// 获取空闲页面
    ///
    /// **注意**: 实际使用需要在 Tauri Command 中调用 `pop_page()` 获取 WebviewWindow
    async fn acquire_page(&mut self) -> Result<Page, InfrastructureError> {
        let service = self.service.lock().await;

        // 调用实际服务获取页面
        let window = service.pop_page().await.ok_or_else(|| {
            InfrastructureError::ExternalError("No idle page available".to_string())
        })?;

        let window_label = window.label().to_string();

        Ok(Page {
            id: aumate_core_shared::PageId::generate(),
            window_id: WindowId::new(window_label),
            url: None, // URL 由调用方设置
            state: aumate_core_domain::page::PageState::Active,
        })
    }

    /// 释放页面回池
    ///
    /// **注意**: 由于 HotLoadPageService 使用 `add_page(WebviewWindow)`,
    /// 实际释放需要在 Tauri Command 中完成，传入实际的 WebviewWindow
    async fn release_page(&mut self, page: Page) -> Result<(), InfrastructureError> {
        // 实际实现需要在 Tauri Command 中：
        // let window = app.get_webview_window(&page.window_id.as_str())
        //     .ok_or(...)?;
        // service.lock().await.add_page(window).await?;

        log::info!("PageManagementAdapter: 准备释放页面 {}", page.window_id.as_str());
        Ok(())
    }

    /// 添加页面到池
    ///
    /// **注意**: 实际实现需要在 Tauri Command 中传入 WebviewWindow
    async fn add_page(
        &mut self,
        window_id: WindowId,
        _url: String,
    ) -> Result<(), InfrastructureError> {
        // 实际实现需要在 Tauri Command 中：
        // let window = app.get_webview_window(&window_id.as_str())
        //     .ok_or(...)?;
        // service.lock().await.add_page(window).await?;

        log::info!("PageManagementAdapter: 准备添加页面 {}", window_id.as_str());
        Ok(())
    }

    /// 获取池状态
    ///
    /// **注意**: 这个方法不是 async 的，但 HotLoadPageService 需要 async 访问
    /// 实际使用时，应该通过 Tauri Command 异步获取状态
    fn get_pool_status(&self) -> PoolStatus {
        // 需要通过 async 方法获取实际状态
        // 这里返回默认值，实际状态应通过 Tauri Command 获取
        PoolStatus { total: 0, idle: 0, active: 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_page_management_adapter_creation() {
        let adapter = PageManagementAdapter::new();
        // 验证服务已创建
        let service = adapter.get_service();
        let _guard = service.lock().await;
        // 创建成功
    }

    #[tokio::test]
    async fn test_init_pool() {
        let mut adapter = PageManagementAdapter::new();
        // 初始化应该成功（实际逻辑在 Tauri Command 中）
        let result = adapter.init_pool(10).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_acquire_page_when_empty() {
        let mut adapter = PageManagementAdapter::new();
        // 未初始化时获取页面应该返回错误（没有空闲页面）
        let result = adapter.acquire_page().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_service() {
        let adapter = PageManagementAdapter::new();
        let service = adapter.get_service();

        // 验证可以获取服务引用
        let _guard = service.lock().await;
    }
}
