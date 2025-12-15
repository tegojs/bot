// UI Control Commands
// Commands for controlling the application UI (command palette, windows, etc.)

use tauri::Manager;

// Helper function to center window precisely
fn center_window_precise(window: &tauri::Window) -> Result<(), String> {
    let monitor = window
        .current_monitor()
        .map_err(|e| format!("Failed to get monitor: {}", e))?
        .ok_or_else(|| "No monitor found".to_string())?;
    
    let scale_factor = monitor.scale_factor();
    let monitor_size = monitor.size();
    let monitor_pos = monitor.position();
    
    // 转换为逻辑像素
    let screen_width = monitor_size.width as f64 / scale_factor;
    let screen_height = monitor_size.height as f64 / scale_factor;
    let monitor_x = monitor_pos.x as f64 / scale_factor;
    let monitor_y = monitor_pos.y as f64 / scale_factor;
    
    // 获取窗口当前尺寸（物理像素）并转换为逻辑像素
    let window_size = window.inner_size().map_err(|e| format!("Failed to get window size: {}", e))?;
    let window_width = window_size.width as f64 / scale_factor;
    let window_height = window_size.height as f64 / scale_factor;
    
    // 计算居中位置
    let target_x = monitor_x + (screen_width - window_width) / 2.0;
    let target_y = monitor_y + (screen_height - window_height) / 2.0;
    
    // 设置位置（使用 LogicalPosition）
    window
        .set_position(tauri::LogicalPosition::new(target_x, target_y))
        .map_err(|e| format!("Failed to set position: {}", e))?;
    
    Ok(())
}

/// Show the command palette window
#[tauri::command]
pub async fn show_command_palette(window: tauri::Window) -> Result<(), String> {
    log::info!("API: show_command_palette called");

    center_window_precise(&window)?;
    window.show().map_err(|e| e.to_string())?;
    window.set_focus().map_err(|e| e.to_string())?;
    Ok(())
}

/// Hide the command palette window
#[tauri::command]
pub async fn hide_command_palette(window: tauri::Window) -> Result<(), String> {
    log::info!("API: hide_command_palette called");

    window.hide().map_err(|e| e.to_string())?;
    Ok(())
}

/// Toggle the command palette window visibility
#[tauri::command]
pub async fn toggle_command_palette(window: tauri::Window) -> Result<(), String> {
    log::info!("API: toggle_command_palette called");

    if window.is_visible().map_err(|e| e.to_string())? {
        window.hide().map_err(|e| e.to_string())?;
    } else {
        center_window_precise(&window)?;
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// Start screenshot mode by showing the screenshot window
#[tauri::command]
pub async fn start_screenshot(app: tauri::AppHandle) -> Result<(), String> {
    log::info!("API: start_screenshot called");

    // Hide the commandpalette window first
    if let Some(commandpalette_window) = app.get_webview_window("commandpalette") {
        let _ = commandpalette_window.hide();
    }

    // Show and focus the screenshot window
    if let Some(screenshot_window) = app.get_webview_window("screenshot") {
        screenshot_window.show().map_err(|e| e.to_string())?;
        screenshot_window.set_focus().map_err(|e| e.to_string())?;
    }

    Ok(())
}

/// Start element scan mode by showing the element scan window
#[tauri::command]
pub async fn start_element_scan(app: tauri::AppHandle) -> Result<(), String> {
    log::info!("API: start_element_scan called");

    // Hide the commandpalette window first
    if let Some(commandpalette_window) = app.get_webview_window("commandpalette") {
        let _ = commandpalette_window.hide();
    }

    // Check if elementscan window exists, if not we need to handle it gracefully
    // Since we defined it in tauri.conf.json, it should exist
    if let Some(elementscan_window) = app.get_webview_window("elementscan") {
        elementscan_window.show().map_err(|e| e.to_string())?;
        elementscan_window.set_focus().map_err(|e| e.to_string())?;
        elementscan_window.set_always_on_top(true).map_err(|e| e.to_string())?;
        log::info!("Element scan window shown and focused");
    } else {
        log::warn!("Element scan window not found, it should be defined in tauri.conf.json");
        return Err("Element scan window not found".to_string());
    }

    Ok(())
}