// 热键管理相关 Tauri Commands
use crate::state::AppState;
use aumate_core_shared::{ApiError, WindowId};
use tauri::{AppHandle, State, WebviewWindow};

/// 开始监听键盘
#[tauri::command]
pub async fn listen_key_start(
    state: State<'_, AppState>,
    app: AppHandle,
    window: WebviewWindow,
) -> Result<(), String> {
    log::info!("API: listen_key_start called");

    let key_service = state.hotkey_listener.get_key_service();
    let mut service = key_service.lock().await;
    let result = service.start(app, window.as_ref().window());
    result.map_err(|e| e.to_string())
}

/// 停止监听键盘
#[tauri::command]
pub async fn listen_key_stop(
    state: State<'_, AppState>,
    window_label: String,
) -> Result<(), String> {
    log::info!("API: listen_key_stop called, window_label={}", window_label);

    let key_service = state.hotkey_listener.get_key_service();
    let mut service = key_service.lock().await;
    let result = service.stop_by_window_label(&window_label);
    result.map_err(|e| e.to_string())
}

/// 开始监听鼠标
#[tauri::command]
pub async fn listen_mouse_start(
    state: State<'_, AppState>,
    app: AppHandle,
    window: WebviewWindow,
) -> Result<(), String> {
    log::info!("API: listen_mouse_start called");

    let mouse_service = state.hotkey_listener.get_mouse_service();
    let mut service = mouse_service.lock().await;
    let result = service.start(app, window.as_ref().window());
    result.map_err(|e| e.to_string())
}

/// 停止监听鼠标
#[tauri::command]
pub async fn listen_mouse_stop(
    state: State<'_, AppState>,
    window_label: String,
) -> Result<(), String> {
    log::info!("API: listen_mouse_stop called, window_label={}", window_label);

    let mouse_service = state.hotkey_listener.get_mouse_service();
    let mut service = mouse_service.lock().await;
    let result = service.stop_by_window_label(&window_label);
    result.map_err(|e| e.to_string())
}
