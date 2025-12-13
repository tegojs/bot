// UI Control Commands
// Commands for controlling the application UI (command palette, windows, etc.)

use tauri::Manager;

/// Show the command palette window
#[tauri::command]
pub async fn show_command_palette(window: tauri::Window) -> Result<(), String> {
    log::info!("API: show_command_palette called");

    window.center().map_err(|e| e.to_string())?;
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
        window.center().map_err(|e| e.to_string())?;
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// Start screenshot mode by showing the screenshot window
#[tauri::command]
pub async fn start_screenshot(app: tauri::AppHandle) -> Result<(), String> {
    log::info!("API: start_screenshot called");

    // Hide the main window first
    if let Some(main_window) = app.get_webview_window("main") {
        let _ = main_window.hide();
    }

    // Show and focus the screenshot window
    if let Some(screenshot_window) = app.get_webview_window("screenshot") {
        screenshot_window.show().map_err(|e| e.to_string())?;
        screenshot_window.set_focus().map_err(|e| e.to_string())?;
    }

    Ok(())
}
