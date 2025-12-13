use aumate_core_shared::{PageId, WindowId};
use serde::{Deserialize, Serialize};

/// 页面状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PageState {
    /// 空闲状态
    Idle,
    /// 加载中
    Loading,
    /// 就绪
    Ready,
    /// 活跃
    Active,
}

/// 页面
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Page {
    pub id: PageId,
    pub window_id: WindowId,
    pub state: PageState,
    pub url: Option<String>,
}

impl Page {
    pub fn new(id: PageId, window_id: WindowId) -> Self {
        Self { id, window_id, state: PageState::Idle, url: None }
    }

    pub fn with_url(mut self, url: String) -> Self {
        self.url = Some(url);
        self
    }

    pub fn set_state(&mut self, state: PageState) {
        self.state = state;
    }

    pub fn is_ready(&self) -> bool {
        self.state == PageState::Ready || self.state == PageState::Active
    }
}

/// 页面池状态
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PoolStatus {
    pub total: usize,
    pub idle: usize,
    pub active: usize,
}

impl PoolStatus {
    pub fn new(total: usize, idle: usize, active: usize) -> Self {
        Self { total, idle, active }
    }

    pub fn available(&self) -> usize {
        self.idle
    }

    pub fn is_full(&self) -> bool {
        self.active >= self.total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_creation() {
        let page = Page::new(PageId::generate(), WindowId::new("test".to_string()));
        assert_eq!(page.state, PageState::Idle);
        assert!(page.url.is_none());
    }

    #[test]
    fn test_page_with_url() {
        let page = Page::new(PageId::generate(), WindowId::new("test".to_string()))
            .with_url("https://example.com".to_string());
        assert_eq!(page.url, Some("https://example.com".to_string()));
    }

    #[test]
    fn test_pool_status() {
        let status = PoolStatus::new(10, 5, 5);
        assert_eq!(status.available(), 5);
        assert!(!status.is_full());

        let full_status = PoolStatus::new(10, 0, 10);
        assert!(full_status.is_full());
    }
}
