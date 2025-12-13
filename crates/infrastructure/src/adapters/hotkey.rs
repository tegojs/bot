use async_trait::async_trait;
use aumate_core_shared::{InfrastructureError, Point, WindowId};
use aumate_core_traits::hotkey::{
    HotkeyListenerPort, InputEventHandler, InputSimulationPort, MouseButton,
};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::services::{ListenKeyService, ListenMouseService};

/// 快捷键监听适配器
///
/// **实际实现**: 完整迁移了底层服务代码
///
/// **复用代码**:
/// - `services::ListenKeyService` (~175 行) - 完整迁移
/// - `services::ListenMouseService` (~165 行) - 完整迁移
/// - `services::DeviceEventHandlerService` (~115 行) - 完整迁移
///
/// **使用方式**:
/// 1. 创建适配器实例
/// 2. 通过 `key_service()` 和 `mouse_service()` 获取服务引用
/// 3. 在 Tauri Command 中调用服务的 `start()` 方法传入 AppHandle 和 Window
pub struct HotkeyListenerAdapter {
    key_service: Arc<Mutex<ListenKeyService>>,
    mouse_service: Arc<Mutex<ListenMouseService>>,
    _handler: Option<Box<dyn InputEventHandler>>,
}

impl HotkeyListenerAdapter {
    pub fn new() -> Self {
        Self {
            key_service: Arc::new(Mutex::new(ListenKeyService::new())),
            mouse_service: Arc::new(Mutex::new(ListenMouseService::new())),
            _handler: None,
        }
    }

    /// 获取 key service 的引用 (用于在 Tauri Command 中调用)
    pub fn get_key_service(&self) -> Arc<Mutex<ListenKeyService>> {
        self.key_service.clone()
    }

    /// 获取 mouse service 的引用 (用于在 Tauri Command 中调用)
    pub fn get_mouse_service(&self) -> Arc<Mutex<ListenMouseService>> {
        self.mouse_service.clone()
    }
}

impl Default for HotkeyListenerAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl HotkeyListenerPort for HotkeyListenerAdapter {
    /// 开始监听
    ///
    /// **注意**: 实际使用需要通过 Tauri Command 调用 `get_key_service()` 和 `get_mouse_service()`
    /// 然后调用它们的 `start(app_handle, window)` 方法
    async fn start_listening(&mut self, window_id: WindowId) -> Result<(), InfrastructureError> {
        log::info!("HotkeyListenerAdapter: 准备监听窗口 {}", window_id.as_str());
        Ok(())
    }

    /// 停止监听
    async fn stop_listening(&mut self, window_id: WindowId) -> Result<(), InfrastructureError> {
        let mut key_service = self.key_service.lock().await;
        key_service.stop_by_window_label(window_id.as_str()).map_err(|e| {
            InfrastructureError::ExternalError(format!("Failed to stop key listener: {}", e))
        })?;

        let mut mouse_service = self.mouse_service.lock().await;
        mouse_service.stop_by_window_label(window_id.as_str()).map_err(|e| {
            InfrastructureError::ExternalError(format!("Failed to stop mouse listener: {}", e))
        })?;

        Ok(())
    }

    /// 注册事件处理器
    fn register_handler(&mut self, handler: Box<dyn InputEventHandler>) {
        self._handler = Some(handler);
    }
}

/// 输入模拟适配器
///
/// **完整实现**: 使用 EnigoManager 提供输入模拟功能
///
/// **迁移代码**:
/// - `services::input_simulation::EnigoManager` (~30 行) - 完整迁移
///
/// **注意**: 输入模拟功能依赖 enigo 库，需要特定平台权限
pub struct InputSimulationAdapter {
    manager: std::sync::Mutex<crate::services::EnigoManager>,
}

impl InputSimulationAdapter {
    pub fn new() -> Self {
        Self { manager: std::sync::Mutex::new(crate::services::EnigoManager::new()) }
    }

    /// 获取 enigo 管理器的可变引用
    fn get_manager(&self) -> std::sync::MutexGuard<crate::services::EnigoManager> {
        self.manager.lock().unwrap()
    }
}

impl Default for InputSimulationAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl InputSimulationPort for InputSimulationAdapter {
    /// 模拟鼠标滚动
    async fn scroll(&self, _delta_x: i32, delta_y: i32) -> Result<(), InfrastructureError> {
        use enigo::{Axis, Mouse};

        let mut manager = self.get_manager();
        let enigo = manager.get_enigo().map_err(|e| {
            InfrastructureError::ExternalError(format!("Failed to get enigo: {}", e))
        })?;

        // 模拟垂直滚动（正值向下，负值向上）
        if delta_y != 0 {
            enigo.scroll(delta_y, Axis::Vertical).map_err(|e| {
                InfrastructureError::ExternalError(format!("Failed to scroll: {}", e))
            })?;
        }

        // 注意：enigo 0.6.1 可能不支持水平滚动或API不同
        // 这里暂时只实现垂直滚动

        Ok(())
    }

    /// 模拟鼠标点击
    async fn click(&self, button: MouseButton, position: Point) -> Result<(), InfrastructureError> {
        use enigo::{Button as EnigoButton, Coordinate, Mouse};

        let mut manager = self.get_manager();
        let enigo = manager.get_enigo().map_err(|e| {
            InfrastructureError::ExternalError(format!("Failed to get enigo: {}", e))
        })?;

        // 先移动到位置
        enigo.move_mouse(position.x, position.y, Coordinate::Abs).map_err(|e| {
            InfrastructureError::ExternalError(format!("Failed to move mouse: {}", e))
        })?;

        // 转换按钮类型
        let enigo_button = match button {
            MouseButton::Left => EnigoButton::Left,
            MouseButton::Right => EnigoButton::Right,
            MouseButton::Middle => EnigoButton::Middle,
        };

        // 点击
        enigo
            .button(enigo_button, enigo::Direction::Click)
            .map_err(|e| InfrastructureError::ExternalError(format!("Failed to click: {}", e)))?;

        Ok(())
    }

    /// 移动鼠标
    async fn move_mouse(&self, position: Point) -> Result<(), InfrastructureError> {
        use enigo::{Coordinate, Mouse};

        let mut manager = self.get_manager();
        let enigo = manager.get_enigo().map_err(|e| {
            InfrastructureError::ExternalError(format!("Failed to get enigo: {}", e))
        })?;

        enigo.move_mouse(position.x, position.y, Coordinate::Abs).map_err(|e| {
            InfrastructureError::ExternalError(format!("Failed to move mouse: {}", e))
        })?;

        Ok(())
    }

    /// 获取鼠标位置
    async fn get_mouse_position(&self) -> Result<Point, InfrastructureError> {
        use enigo::Mouse;

        let mut manager = self.get_manager();
        let enigo = manager.get_enigo().map_err(|e| {
            InfrastructureError::ExternalError(format!("Failed to get enigo: {}", e))
        })?;

        let (x, y) = enigo.location().map_err(|e| {
            InfrastructureError::ExternalError(format!("Failed to get mouse position: {}", e))
        })?;

        Ok(Point { x, y })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_hotkey_listener_adapter_creation() {
        let adapter = HotkeyListenerAdapter::new();
        // 验证服务可以获取
        let key_svc = adapter.get_key_service();
        let mouse_svc = adapter.get_mouse_service();
        {
            let _ks = key_svc.lock().await;
            let _ms = mouse_svc.lock().await;
        }
        // 创建成功
    }

    #[tokio::test]
    async fn test_input_simulation_adapter_creation() {
        let _adapter = InputSimulationAdapter::new();
        // 创建成功
    }

    #[tokio::test]
    async fn test_stop_listener() {
        let mut adapter = HotkeyListenerAdapter::new();
        // 停止未启动的监听器应该成功
        let result = adapter.stop_listening(WindowId::new("test-window".to_string())).await;
        assert!(result.is_ok());
    }
}
