use crate::state::AppState;
use aumate_application::dto::ScannableElementDto;
use aumate_core_shared::ApiError;
use tauri::State;

/// 扫描屏幕上的可交互元素
#[tauri::command]
pub async fn scan_screen_elements(
    state: State<'_, AppState>,
) -> Result<Vec<ScannableElementDto>, String> {
    log::info!("API: scan_screen_elements called");

    state
        .scan_elements_use_case
        .execute()
        .await
        .map_err(|e| {
            let api_error: ApiError = e.into();
            api_error.to_string()
        })
}

/// 触发元素操作（点击或聚焦）
#[tauri::command]
pub async fn trigger_element_action(
    element_id: String,
    action_type: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    log::info!(
        "API: trigger_element_action called - element_id: {}, action_type: {}",
        element_id,
        action_type
    );

    // 解析操作类型（参数验证）
    let action = aumate_application::use_cases::ElementActionType::from_str(&action_type)
        .map_err(|e| {
            let api_error: ApiError = e.into();
            api_error.to_string()
        })?;

    // 调用统一的业务逻辑用例
    state
        .trigger_element_action_use_case
        .execute(&element_id, action)
        .await
        .map_err(|e| {
            let api_error: ApiError = e.into();
            api_error.to_string()
        })
}

