// Global Shortcut Management Commands
use crate::state::AppState;
use aumate_core_shared::ApiError;
use tauri::State;

/// 注册全局快捷键
#[tauri::command]
pub async fn register_global_shortcut(
    state: State<'_, AppState>,
    shortcut: String,
) -> Result<(), String> {
    log::info!("API: register_global_shortcut called - shortcut: {}", shortcut);

    state
        .register_global_shortcut
        .execute(shortcut)
        .await
        .map_err(|e| {
            let api_error: ApiError = e.into();
            api_error.to_string()
        })
}

/// 注销全局快捷键
#[tauri::command]
pub async fn unregister_global_shortcut(
    state: State<'_, AppState>,
    shortcut: String,
) -> Result<(), String> {
    log::info!("API: unregister_global_shortcut called - shortcut: {}", shortcut);

    state
        .unregister_global_shortcut
        .execute(shortcut)
        .await
        .map_err(|e| {
            let api_error: ApiError = e.into();
            api_error.to_string()
        })
}

/// 检查全局快捷键是否可用
#[tauri::command]
pub async fn check_global_shortcut_availability(
    state: State<'_, AppState>,
    shortcut: String,
) -> Result<bool, String> {
    log::info!("API: check_global_shortcut_availability called - shortcut: {}", shortcut);

    state
        .check_global_shortcut_availability
        .execute(shortcut)
        .await
        .map_err(|e| {
            let api_error: ApiError = e.into();
            api_error.to_string()
        })
}
