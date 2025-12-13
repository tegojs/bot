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
pub async fn get_window_elements(state: State<'_, AppState>) -> Result<Vec<WindowElementDto>, String> {
    log::info!("API: get_window_elements called");

    let windows = state
        .get_window_elements
        .execute()
        .await
        .map_err(|e| {
            let api_error: ApiError = e.into();
            api_error.to_string()
        })?;

    // 转换为 DTO
    let window_dtos: Vec<WindowElementDto> = windows.into_iter().map(Into::into).collect();
    
    Ok(window_dtos)
}

/// 带动画的调整窗口大小并居中（完整 Rust 实现）
/// 
/// 此命令会：
/// 1. 获取当前窗口状态和显示器信息
/// 2. 在 Rust 端执行平滑动画
/// 3. 正确处理逻辑像素和物理像素转换
/// 4. 支持多显示器场景
#[tauri::command]
pub async fn animate_resize_and_center(
    window: tauri::Window,
    target_width: f64,
    target_height: f64,
    duration: u64,
) -> Result<(), String> {
    log::info!(
        "API: animate_resize_and_center called, target={}x{}, duration={}ms",
        target_width,
        target_height,
        duration
    );

    // 获取当前显示器信息
    let monitor = window
        .current_monitor()
        .map_err(|e| format!("Failed to get current monitor: {}", e))?
        .ok_or_else(|| "No monitor found".to_string())?;

    let scale_factor = monitor.scale_factor();
    
    // 获取显示器尺寸（物理像素）并转换为逻辑像素
    let monitor_size = monitor.size();
    let screen_width = monitor_size.width as f64 / scale_factor;
    let screen_height = monitor_size.height as f64 / scale_factor;
    
    // 获取显示器位置（物理像素）并转换为逻辑像素
    let monitor_pos = monitor.position();
    let monitor_x = monitor_pos.x as f64 / scale_factor;
    let monitor_y = monitor_pos.y as f64 / scale_factor;
    
    log::debug!(
        "Monitor: size={}x{} (logical), pos=({}, {}), scale={}",
        screen_width,
        screen_height,
        monitor_x,
        monitor_y,
        scale_factor
    );

    // 计算目标居中位置（逻辑像素）
    let target_x = monitor_x + (screen_width - target_width) / 2.0;
    let target_y = monitor_y + (screen_height - target_height) / 2.0;
    
    log::debug!("Target: pos=({}, {}), size={}x{} (all logical)", target_x, target_y, target_width, target_height);

    // 获取当前窗口尺寸（物理像素）并转换为逻辑像素
    let current_size = window
        .inner_size()
        .map_err(|e| format!("Failed to get window size: {}", e))?;
    let current_width = current_size.width as f64 / scale_factor;  // 转换为逻辑像素
    let current_height = current_size.height as f64 / scale_factor; // 转换为逻辑像素
    
    // 获取当前窗口位置（物理像素）并转换为逻辑像素
    let current_pos = window
        .outer_position()
        .map_err(|e| format!("Failed to get window position: {}", e))?;
    let current_x = current_pos.x as f64 / scale_factor;
    let current_y = current_pos.y as f64 / scale_factor;
    
    log::debug!(
        "Current: pos=({}, {}), size={}x{} (all logical)",
        current_x,
        current_y,
        current_width,
        current_height
    );

    // 如果已经是目标大小和位置，直接返回
    if (current_width - target_width).abs() < 1.0
        && (current_height - target_height).abs() < 1.0
        && (current_x - target_x).abs() < 1.0
        && (current_y - target_y).abs() < 1.0
    {
        log::info!("Window already at target position and size");
        return Ok(());
    }

    // 缓动函数 (easeInOutCubic)
    let ease_in_out_cubic = |t: f64| -> f64 {
        if t < 0.5 {
            4.0 * t * t * t
        } else {
            1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
        }
    };

    // 动画参数
    let fps = 60;
    let frame_duration = std::time::Duration::from_millis(1000 / fps);
    let total_frames = (duration as f64 / (1000.0 / fps as f64)).round() as u64;
    
    log::debug!("Animation: {} frames at {}fps", total_frames, fps);

    // 执行动画
    for frame in 0..=total_frames {
        let progress = frame as f64 / total_frames as f64;
        let eased_progress = ease_in_out_cubic(progress);
        
        // 计算当前帧的尺寸（逻辑像素）
        let frame_width = current_width + (target_width - current_width) * eased_progress;
        let frame_height = current_height + (target_height - current_height) * eased_progress;
        
        // 计算当前帧的位置（逻辑像素）
        let frame_x = current_x + (target_x - current_x) * eased_progress;
        let frame_y = current_y + (target_y - current_y) * eased_progress;
        
        // 设置窗口大小（使用逻辑像素）
        window
            .set_size(tauri::LogicalSize::new(
                frame_width.round(),
                frame_height.round(),
            ))
            .map_err(|e| format!("Failed to set size at frame {}: {}", frame, e))?;
        
        // 设置窗口位置（使用逻辑像素！Tauri 会自动转换）
        window
            .set_position(tauri::LogicalPosition::new(
                frame_x.round(),
                frame_y.round(),
            ))
            .map_err(|e| format!("Failed to set position at frame {}: {}", frame, e))?;
        
        // 等待下一帧
        if frame < total_frames {
            tokio::time::sleep(frame_duration).await;
        }
    }
    
    // 确保最终状态精确
    window
        .set_size(tauri::LogicalSize::new(target_width, target_height))
        .map_err(|e| format!("Failed to set final size: {}", e))?;
    
    window
        .set_position(tauri::LogicalPosition::new(target_x, target_y))
        .map_err(|e| format!("Failed to set final position: {}", e))?;

    log::info!("Animation completed successfully");
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
