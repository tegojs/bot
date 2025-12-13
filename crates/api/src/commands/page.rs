// 页面管理相关 Tauri Commands
use crate::state::AppState;
use aumate_core_shared::ApiError;
use tauri::{State, WebviewWindow};

/// 添加页面到热加载管理
#[tauri::command]
pub async fn add_page(state: State<'_, AppState>, window: WebviewWindow) -> Result<(), String> {
    log::info!("API: add_page called, label={}", window.label());

    let page_service = state.page_management.get_service();
    let mut service = page_service.lock().await;
    let result = service.add_page(window);
    result.await.map_err(|e| e.to_string())
}

/// 从热加载管理移除页面  
#[tauri::command]
pub async fn remove_page(state: State<'_, AppState>, page_id: String) -> Result<(), String> {
    log::info!("API: remove_page called, page_id={}", page_id);

    // 调用 page management 服务移除页面
    state.page_management.remove_page(page_id).await.map_err(|e| e.to_string())
}
