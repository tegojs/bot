/// 键盘监听服务
///
/// **复用**: `app-services/src/listen_key_service.rs` (~190 行)
///
/// 监听全局键盘事件并通过 Tauri 事件系统分发
use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

use device_query::Keycode;
use serde::Serialize;
use tauri::{AppHandle, Emitter, Window};

use super::device_events::DeviceEventHandlerService;

pub struct ListenKeyService {
    _key_down_guard: Arc<Mutex<Option<Box<dyn std::any::Any + Send>>>>,
    _key_up_guard: Arc<Mutex<Option<Box<dyn std::any::Any + Send>>>>,
    window_label_set: Arc<Mutex<HashSet<String>>>,
    device_event_handler: Arc<Mutex<DeviceEventHandlerService>>,
}

#[derive(Serialize, Clone)]
pub struct ListenKeyDownEvent {
    pub key: usize,
    pub key_text: String,
}

#[derive(Serialize, Clone)]
pub struct ListenKeyUpEvent {
    pub key: usize,
    pub key_text: String,
}

impl ListenKeyService {
    pub fn new() -> Self {
        Self {
            _key_down_guard: Arc::new(Mutex::new(None)),
            _key_up_guard: Arc::new(Mutex::new(None)),
            window_label_set: Arc::new(Mutex::new(HashSet::new())),
            device_event_handler: Arc::new(Mutex::new(DeviceEventHandlerService::new())),
        }
    }

    pub fn start(&mut self, app_handle: AppHandle, window: Window) -> Result<(), String> {
        let mut window_label_set_lock = self.window_label_set.lock().map_err(|err| {
            format!("[ListenKeyService::start] Failed to lock window_label_set: {}", err)
        })?;
        window_label_set_lock.insert(window.label().to_owned());

        let mut key_down_guard_lock = self._key_down_guard.lock().map_err(|err| {
            format!("[ListenKeyService::start] Failed to lock key_down_guard: {}", err)
        })?;

        let mut device_event_handler = self.device_event_handler.lock().unwrap();
        if key_down_guard_lock.is_none() {
            let key_down_app_handle = app_handle.clone();
            *key_down_guard_lock =
                Some(Box::new(device_event_handler.on_key_down(move |key: &Keycode| {
                    match key_down_app_handle.emit(
                        "listen-key-service:key-down",
                        ListenKeyDownEvent { key: *key as usize, key_text: key.to_string() },
                    ) {
                        Ok(_) => {}
                        Err(_) => {
                            log::error!(
                                "[ListenKeyService::on_key_down] Failed to emit listen-key-service"
                            );
                        }
                    };
                })?));
        }

        let mut key_up_guard_lock = self._key_up_guard.lock().map_err(|err| {
            format!("[ListenKeyService::start] Failed to lock key_up_guard: {}", err)
        })?;

        // macOS 部分键如 Shift 等， key up 在鼠标移动后会触发，无法保持按下状态
        if key_up_guard_lock.is_none() {
            let key_up_app_handle = app_handle.clone();
            *key_up_guard_lock =
                Some(Box::new(device_event_handler.on_key_up(move |key: &Keycode| {
                    match key_up_app_handle.emit(
                        "listen-key-service:key-up",
                        ListenKeyUpEvent { key: *key as usize, key_text: key.to_string() },
                    ) {
                        Ok(_) => {}
                        Err(_) => {
                            log::error!(
                                "[ListenKeyService::on_key_up] Failed to emit listen-key-service"
                            );
                        }
                    };
                })?));
        }

        Ok(())
    }

    fn stop_core(
        key_down_guard: &Arc<Mutex<Option<Box<dyn std::any::Any + Send>>>>,
        key_up_guard: &Arc<Mutex<Option<Box<dyn std::any::Any + Send>>>>,
        device_event_handler: &Arc<Mutex<DeviceEventHandlerService>>,
        window_label_set: &Arc<Mutex<HashSet<String>>>,
        window_label: &str,
    ) -> Result<(), String> {
        let mut window_label_set_lock = window_label_set.lock().map_err(|err| {
            format!("[ListenKeyService::stop_core] Failed to lock window_label_set: {}", err)
        })?;
        window_label_set_lock.remove(window_label);

        if !window_label_set_lock.is_empty() {
            return Ok(());
        }

        // 没有窗口监听了，清除监听
        let mut key_down_guard_lock = key_down_guard.lock().map_err(|err| {
            format!("[ListenKeyService::stop_core] Failed to lock key_down_guard: {}", err)
        })?;
        *key_down_guard_lock = None;

        let mut key_up_guard_lock = key_up_guard.lock().map_err(|err| {
            format!("[ListenKeyService::stop_core] Failed to lock key_up_guard: {}", err)
        })?;
        *key_up_guard_lock = None;

        let mut device_event_handler_lock = device_event_handler.lock().map_err(|_| {
            "[ListenKeyService::stop_core] Failed to lock device_event_handler".to_string()
        })?;
        device_event_handler_lock.release();

        Ok(())
    }

    pub fn stop_by_window_label(&mut self, window_label: &str) -> Result<(), String> {
        Self::stop_core(
            &self._key_down_guard,
            &self._key_up_guard,
            &self.device_event_handler,
            &self.window_label_set,
            window_label,
        )
    }
}

impl Default for ListenKeyService {
    fn default() -> Self {
        Self::new()
    }
}
