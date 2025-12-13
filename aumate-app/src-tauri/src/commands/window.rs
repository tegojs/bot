// 窗口管理相关 Tauri Commands
use crate::state::AppState;
use aumate_application::dto::{
    CreateWindowRequest, CreateWindowResponse, DragWindowRequest, ResizeWindowRequest,
};
use aumate_core_shared::{ApiError, Point, WindowId};
use tauri::State;

/// 创建窗口
#[tauri::command]
pub async fn create_window(
    state: State<'_, AppState>,
    label: String,
    title: String,
    url: String,
    width: Option<f64>,
    height: Option<f64>,
) -> Result<CreateWindowResponse, String> {
    log::info!("API: create_window called, label={}", label);

    let request = CreateWindowRequest { label, title, url, width, height };

    state.window_management.create_window(request).await.map_err(|e| {
        let api_error: ApiError = e.into();
        api_error.to_string()
    })
}

/// 拖动窗口
#[tauri::command]
pub async fn drag_window(
    state: State<'_, AppState>,
    window_id: String,
    delta_x: i32,
    delta_y: i32,
) -> Result<(), String> {
    log::info!("API: drag_window called, window_id={}", window_id);

    let request = DragWindowRequest {
        window_id: WindowId::new(window_id),
        delta: Point::new(delta_x, delta_y),
    };

    state.window_management.drag_window(request).await.map_err(|e| {
        let api_error: ApiError = e.into();
        api_error.to_string()
    })
}

/// 调整窗口大小
#[tauri::command]
pub async fn resize_window(
    state: State<'_, AppState>,
    window_id: String,
    side: String,
    delta: i32,
) -> Result<(), String> {
    log::info!("API: resize_window called, window_id={}", window_id);

    let request = ResizeWindowRequest { window_id: WindowId::new(window_id), side, delta };

    state.window_management.resize_window(request).await.map_err(|e| {
        let api_error: ApiError = e.into();
        api_error.to_string()
    })
}

/// 固定窗口（置顶）
#[tauri::command]
pub async fn pin_window(state: State<'_, AppState>, window_id: String) -> Result<(), String> {
    log::info!("API: pin_window called, window_id={}", window_id);

    state.window_management.set_always_on_top(WindowId::new(window_id), true).await.map_err(|e| {
        let api_error: ApiError = e.into();
        api_error.to_string()
    })
}

/// 取消固定窗口
#[tauri::command]
pub async fn unpin_window(state: State<'_, AppState>, window_id: String) -> Result<(), String> {
    log::info!("API: unpin_window called, window_id={}", window_id);

    state.window_management.set_always_on_top(WindowId::new(window_id), false).await.map_err(|e| {
        let api_error: ApiError = e.into();
        api_error.to_string()
    })
}

/// 关闭窗口
#[tauri::command]
pub async fn close_window(state: State<'_, AppState>, window_id: String) -> Result<(), String> {
    log::info!("API: close_window called, window_id={}", window_id);

    state.window_management.close_window(WindowId::new(window_id)).await.map_err(|e| {
        let api_error: ApiError = e.into();
        api_error.to_string()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_window_request() {
        let request = CreateWindowRequest {
            label: "test".to_string(),
            title: "Test Window".to_string(),
            url: "index.html".to_string(),
            width: Some(800.0),
            height: Some(600.0),
        };
        assert_eq!(request.label, "test");
        assert_eq!(request.width, Some(800.0));
    }
}
