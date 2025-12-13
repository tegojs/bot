// UI 自动化适配器
use async_trait::async_trait;
use aumate_core_shared::{InfrastructureError, Point};
use aumate_core_traits::window::{UIAutomationPort, UIElement};
use std::sync::Arc;
use tokio::sync::Mutex;

#[cfg(target_os = "macos")]
use crate::platform::macos::ui_automation::UIElements;

#[cfg(target_os = "windows")]
use crate::platform::windows::ui_automation::UIElements;

#[cfg(target_os = "linux")]
use crate::platform::linux::ui_automation::UIElements;

/// UI 自动化适配器
pub struct UIAutomationAdapter {
    elements: Arc<Mutex<UIElements>>,
}

impl UIAutomationAdapter {
    pub fn new() -> Self {
        log::info!("Creating UIAutomationAdapter");
        Self { elements: Arc::new(Mutex::new(UIElements::new())) }
    }
}

impl Default for UIAutomationAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl UIAutomationPort for UIAutomationAdapter {
    async fn init_ui_elements(&mut self) -> Result<(), InfrastructureError> {
        log::info!("UIAutomationAdapter: init_ui_elements");

        let mut elements = self.elements.lock().await;
        elements.init().map_err(|e| InfrastructureError::PlatformOperationFailed(e))
    }

    async fn init_ui_elements_cache(&mut self) -> Result<(), InfrastructureError> {
        log::info!("UIAutomationAdapter: init_ui_elements_cache");

        let mut elements = self.elements.lock().await;
        elements.init_cache().map_err(|e| InfrastructureError::PlatformOperationFailed(e))
    }

    async fn get_element_from_position(
        &self,
        position: Point,
    ) -> Result<Vec<UIElement>, InfrastructureError> {
        log::info!(
            "UIAutomationAdapter: get_element_from_position at ({}, {})",
            position.x,
            position.y
        );

        let elements = self.elements.lock().await;
        elements
            .get_elements_at_position(position.x, position.y)
            .map_err(|e| InfrastructureError::PlatformOperationFailed(e))
    }
}

// 辅助方法
impl UIAutomationAdapter {
    /// 清除 UI 元素缓存
    pub async fn clear_cache(&self) {
        log::info!("UIAutomationAdapter: clearing cache");
        let elements = self.elements.lock().await;
        elements.clear_cache();
    }

    /// 获取焦点元素
    pub async fn _get_focused_element(&self) -> Result<Option<UIElement>, InfrastructureError> {
        log::info!("UIAutomationAdapter: get_focused_element");

        // TODO: 实现焦点元素获取
        Ok(None)
    }

    /// 获取所有窗口
    pub async fn get_all_windows(
        &self,
    ) -> Result<Vec<crate::platform::ui_automation::WindowElement>, InfrastructureError> {
        log::info!("UIAutomationAdapter: get_all_windows");
        crate::platform::ui_automation::get_all_windows()
            .map_err(|e| InfrastructureError::PlatformOperationFailed(e))
    }

    /// 获取指定位置的窗口
    pub async fn get_window_at_point(
        &self,
        x: i32,
        y: i32,
    ) -> Result<Option<crate::platform::ui_automation::WindowElement>, InfrastructureError> {
        log::info!("UIAutomationAdapter: get_window_at_point at ({}, {})", x, y);
        crate::platform::ui_automation::get_window_at_point(x, y)
            .map_err(|e| InfrastructureError::PlatformOperationFailed(e))
    }

    /// 切换到窗口
    pub async fn switch_to_window(&self, window_id: u32) -> Result<(), InfrastructureError> {
        log::info!("UIAutomationAdapter: switch_to_window {}", window_id);
        crate::platform::ui_automation::switch_to_window(window_id)
            .map_err(|e| InfrastructureError::PlatformOperationFailed(e))
    }

    /// 关闭窗口
    pub async fn close_window(&self, window_id: u32) -> Result<(), InfrastructureError> {
        log::info!("UIAutomationAdapter: close_window {}", window_id);
        crate::platform::ui_automation::close_window(window_id)
            .map_err(|e| InfrastructureError::PlatformOperationFailed(e))
    }
}
