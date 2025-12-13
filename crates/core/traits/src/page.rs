use async_trait::async_trait;
use aumate_core_shared::{InfrastructureError, WindowId};

// Re-export domain types for convenience
pub use aumate_core_domain::page::{Page, PageState, PoolStatus};

/// 页面管理 Port
///
/// 负责页面池管理
///
/// **实现者**:
/// - `PagePoolAdapter`
#[async_trait]
pub trait PageManagementPort: Send + Sync {
    /// 初始化页面池
    async fn init_pool(&mut self, capacity: usize) -> Result<(), InfrastructureError>;

    /// 获取空闲页面
    async fn acquire_page(&mut self) -> Result<Page, InfrastructureError>;

    /// 释放页面回池
    async fn release_page(&mut self, page: Page) -> Result<(), InfrastructureError>;

    /// 添加页面到池
    async fn add_page(
        &mut self,
        window_id: WindowId,
        url: String,
    ) -> Result<(), InfrastructureError>;

    /// 获取池状态
    fn get_pool_status(&self) -> PoolStatus;
}
