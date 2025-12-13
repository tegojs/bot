/// 设备事件处理服务
///
/// **复用**: `app-services/src/device_event_handler_service.rs` (~87 行)
///
/// 封装 device_query 的事件监听功能，提供跨平台的设备事件处理
use std::time::Duration;

use device_query::{
    CallbackGuard, DeviceEvents, DeviceEventsHandlerInnerThread, Keycode, MouseButton,
    MousePosition,
};

const DEVICE_EVENT_HANDLER_FPS: u64 = 100;

pub struct DeviceEventHandlerService {
    fps: u64,
    device_event_handler: Option<DeviceEventsHandlerInnerThread>,
}

impl DeviceEventHandlerService {
    pub fn new() -> Self {
        Self { fps: DEVICE_EVENT_HANDLER_FPS, device_event_handler: None }
    }

    pub fn set_fps(&mut self, fps: u64) {
        self.fps = fps;
    }

    fn get_device_event_handler(&mut self) -> Result<&DeviceEventsHandlerInnerThread, String> {
        if self.device_event_handler.is_some() {
            return Ok(self.device_event_handler.as_ref().unwrap());
        }

        #[cfg(target_os = "macos")]
        {
            if !macos_accessibility_client::accessibility::application_is_trusted() {
                return Err("[DeviceEventHandlerService] Accessibility is not enabled".to_string());
            }
        }

        let handler = DeviceEventsHandlerInnerThread::new(Duration::from_millis(1000 / self.fps));

        self.device_event_handler = Some(handler);
        Ok(self.device_event_handler.as_ref().unwrap())
    }

    pub fn on_mouse_move<Callback: Fn(&MousePosition) + Sync + Send + 'static>(
        &mut self,
        callback: Callback,
    ) -> Result<CallbackGuard<Callback>, String> {
        Ok(self.get_device_event_handler()?.on_mouse_move(callback))
    }

    pub fn on_mouse_down<Callback: Fn(&MouseButton) + Sync + Send + 'static>(
        &mut self,
        callback: Callback,
    ) -> Result<CallbackGuard<Callback>, String> {
        Ok(self.get_device_event_handler()?.on_mouse_down(callback))
    }

    pub fn on_mouse_up<Callback: Fn(&MouseButton) + Sync + Send + 'static>(
        &mut self,
        callback: Callback,
    ) -> Result<CallbackGuard<Callback>, String> {
        Ok(self.get_device_event_handler()?.on_mouse_up(callback))
    }

    pub fn on_key_down<Callback: Fn(&Keycode) + Sync + Send + 'static>(
        &mut self,
        callback: Callback,
    ) -> Result<CallbackGuard<Callback>, String> {
        Ok(self.get_device_event_handler()?.on_key_down(callback))
    }

    pub fn on_key_up<Callback: Fn(&Keycode) + Sync + Send + 'static>(
        &mut self,
        callback: Callback,
    ) -> Result<CallbackGuard<Callback>, String> {
        Ok(self.get_device_event_handler()?.on_key_up(callback))
    }

    pub fn release(&mut self) {
        self.device_event_handler.take();
    }
}

impl Default for DeviceEventHandlerService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_event_handler_creation() {
        let _service = DeviceEventHandlerService::new();
        // 创建成功
    }

    #[test]
    fn test_set_fps() {
        let mut service = DeviceEventHandlerService::new();
        service.set_fps(60);
        assert_eq!(service.fps, 60);
    }
}
