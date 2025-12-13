// 监视器信息相关 Tauri Commands
use crate::state::AppState;
use aumate_application::dto::monitor::{MonitorInfo, MonitorsResponse};
use aumate_core_shared::ApiError;
use tauri::State;

/// 获取所有监视器信息
#[tauri::command]
pub async fn get_monitors(state: State<'_, AppState>) -> Result<MonitorsResponse, String> {
    log::info!("API: get_monitors called");

    state.get_monitors.execute().await.map_err(|e| {
        let api_error: ApiError = e.into();
        api_error.to_string()
    })
}

/// 获取当前监视器信息
#[tauri::command]
pub async fn get_current_monitor(state: State<'_, AppState>) -> Result<MonitorInfo, String> {
    log::info!("API: get_current_monitor called");

    state.get_current_monitor.execute().await.map_err(|e| {
        let api_error: ApiError = e.into();
        api_error.to_string()
    })
}
