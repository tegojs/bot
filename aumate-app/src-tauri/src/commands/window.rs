// 窗口管理相关 Tauri Commands
use crate::state::AppState;
use aumate_application::dto::{
    CreateWindowRequest, CreateWindowResponse, DragWindowRequest, ResizeWindowRequest,
    WindowElementDto,
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

/// 获取所有窗口元素
#[tauri::command]
pub async fn get_window_elements(
    state: State<'_, AppState>,
) -> Result<Vec<WindowElementDto>, String> {
    log::info!("API: get_window_elements called");

    let windows = state.get_window_elements.execute().await.map_err(|e| {
        let api_error: ApiError = e.into();
        api_error.to_string()
    })?;

    // 转换为 DTO
    let window_dtos: Vec<WindowElementDto> = windows.into_iter().map(Into::into).collect();

    Ok(window_dtos)
}

/// 切换到指定的桌面窗口（使其获得焦点）
#[tauri::command]
pub async fn switch_to_window(state: State<'_, AppState>, window_id: u32) -> Result<(), String> {
    log::info!("API: switch_to_window called, window_id={}", window_id);

    state.switch_to_window.execute(window_id).await.map_err(|e| {
        let api_error: ApiError = e.into();
        api_error.to_string()
    })
}

/// 关闭指定的桌面窗口
#[tauri::command]
pub async fn close_desktop_window(
    state: State<'_, AppState>,
    window_id: u32,
) -> Result<(), String> {
    log::info!("API: close_desktop_window called, window_id={}", window_id);

    state.close_desktop_window.execute(window_id).await.map_err(|e| {
        let api_error: ApiError = e.into();
        api_error.to_string()
    })
}

/// 立即调整窗口大小并居中（无动画）
///
/// 使用 DDD 架构：调用 Application Layer 的 Use Case
#[tauri::command]
pub async fn resize_and_center(
    state: tauri::State<'_, crate::state::AppState>,
    app: tauri::AppHandle,
    window_label: String,
    target_width: f64,
    target_height: f64,
) -> Result<(), String> {
    log::info!(
        "API: resize_and_center called, window={}, target={}x{}",
        window_label,
        target_width,
        target_height
    );

    // 获取窗口 ID
    let window_id = aumate_core_shared::WindowId::new(window_label.clone());

    // 从 app 获取 WebviewWindow并注册
    use tauri::Manager;
    if let Some(webview_window) = app.get_webview_window(&window_label) {
        state.window_layout.register_window(window_id.clone(), webview_window).await;
    } else {
        return Err(format!("Window not found: {}", window_label));
    }

    // 调用 Use Case
    state
        .resize_and_center
        .execute(window_id, target_width, target_height)
        .await
        .map_err(|e| format!("Failed to resize and center: {}", e))?;

    log::info!("Window centered successfully");
    Ok(())
}

/// 带动画的调整窗口大小并居中
///
/// 使用 DDD 架构：调用 Application Layer 的 Use Case
#[tauri::command]
pub async fn animate_resize_and_center(
    state: tauri::State<'_, crate::state::AppState>,
    app: tauri::AppHandle,
    window_label: String,
    target_width: f64,
    target_height: f64,
    duration: u64,
) -> Result<(), String> {
    log::info!(
        "API: animate_resize_and_center called, window={}, target={}x{}, duration={}ms",
        window_label,
        target_width,
        target_height,
        duration
    );

    // 获取窗口 ID
    let window_id = aumate_core_shared::WindowId::new(window_label.clone());

    // 从 app 获取 WebviewWindow并注册
    use tauri::Manager;
    if let Some(webview_window) = app.get_webview_window(&window_label) {
        state.window_layout.register_window(window_id.clone(), webview_window).await;
    } else {
        return Err(format!("Window not found: {}", window_label));
    }

    // 调用 Use Case
    state
        .animate_resize_and_center
        .execute(window_id, target_width, target_height, duration)
        .await
        .map_err(|e| format!("Failed to animate resize and center: {}", e))?;

    log::info!("Animation completed successfully");
    Ok(())
}

/// Set window vibrancy effect (Acrylic/Mica on Windows, vibrancy on macOS)
///
/// Used to temporarily disable vibrancy during animations to avoid visual artifacts.
/// Uses DDD architecture: calls Application Layer's Use Case
#[tauri::command]
pub async fn set_window_vibrancy(
    state: State<'_, AppState>,
    app: tauri::AppHandle,
    window_label: String,
    enabled: bool,
) -> Result<(), String> {
    log::info!(
        "API: set_window_vibrancy called, window={}, enabled={}",
        window_label,
        enabled
    );

    // 获取窗口 ID
    let window_id = aumate_core_shared::WindowId::new(window_label.clone());

    // 从 app 获取 WebviewWindow 并注册到 adapter
    use tauri::Manager;
    if let Some(webview_window) = app.get_webview_window(&window_label) {
        state.window_vibrancy.register_window(window_id.clone(), webview_window).await;
    } else {
        return Err(format!("Window not found: {}", window_label));
    }

    // 调用 Use Case
    state
        .set_window_vibrancy
        .execute(window_id, enabled, None)
        .await
        .map_err(|e| format!("Failed to set vibrancy: {}", e))?;

    log::info!("Window vibrancy set successfully");
    Ok(())
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
