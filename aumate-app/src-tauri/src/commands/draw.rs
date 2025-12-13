use tauri::{AppHandle, Emitter, Manager, WebviewUrl, WebviewWindowBuilder};

/// 创建或显示截图窗口
#[tauri::command]
pub async fn create_draw_window(app: AppHandle) -> Result<(), String> {
    log::info!("API: create_draw_window called");

    // 尝试获取现有窗口
    if let Some(window) = app.get_webview_window("draw") {
        // 窗口已存在，只需显示
        log::info!("Draw window exists, showing it");
        let _ = window.show();
        let _ = window.set_focus();
        
        // 重新设置窗口层级以覆盖菜单栏和 Dock
        #[cfg(target_os = "macos")]
        set_window_above_menubar(&window)?;
        
        let _ = window.emit("start-screenshot", ());
        return Ok(());
    }

    // 获取当前监视器信息
    let monitor = app
        .primary_monitor()
        .map_err(|e| format!("Failed to get primary monitor: {}", e))?
        .ok_or_else(|| "No primary monitor found".to_string())?;

    let position = monitor.position();
    let size = monitor.size();
    let scale_factor = monitor.scale_factor();

    // 转换为逻辑像素
    let logical_x = position.x as f64 / scale_factor;
    let logical_y = position.y as f64 / scale_factor;
    let logical_width = size.width as f64 / scale_factor;
    let logical_height = size.height as f64 / scale_factor;

    log::info!(
        "Creating draw window at ({}, {}) with size {}x{}",
        logical_x,
        logical_y,
        logical_width,
        logical_height
    );

    // 创建新窗口
    let window = WebviewWindowBuilder::new(
        &app,
        "draw",
        WebviewUrl::App("pages/draw.html".into()),
    )
    .title("Screenshot Editor")
    .position(logical_x, logical_y)
    .inner_size(logical_width, logical_height)
    .decorations(false)
    .transparent(true)
    .resizable(false)
    .maximizable(false)
    .minimizable(false)
    .shadow(false)
    .skip_taskbar(true)
    .always_on_top(true)
    .focused(true)
    .build()
    .map_err(|e| format!("Failed to create window: {}", e))?;

    // macOS: 设置窗口层级以覆盖菜单栏和 Dock
    #[cfg(target_os = "macos")]
    set_window_above_menubar(&window)?;

    // 触发截图事件
    let _ = window.emit("start-screenshot", ());

    Ok(())
}

/// macOS: 设置窗口层级以覆盖菜单栏和 Dock
#[cfg(target_os = "macos")]
fn set_window_above_menubar(window: &tauri::WebviewWindow) -> Result<(), String> {
    use cocoa::appkit::{NSMainMenuWindowLevel, NSWindow, NSWindowCollectionBehavior};
    use cocoa::base::id;
    
    // 获取 NSWindow 指针，转换为 usize 以便在线程间传递
    let ns_window_ptr = window.ns_window()
        .map_err(|e| format!("Failed to get NSWindow: {:?}", e))? as usize;
    
    window.run_on_main_thread(move || unsafe {
        let ns_window = ns_window_ptr as id;
        
        // 设置窗口层级高于菜单栏
        // NSMainMenuWindowLevel = 24, 我们使用 25 确保覆盖所有
        ns_window.setLevel_((NSMainMenuWindowLevel + 1) as i64);
        
        // 设置窗口行为：可以出现在所有空间、静止、全屏辅助
        let behavior = NSWindowCollectionBehavior::NSWindowCollectionBehaviorCanJoinAllSpaces
            | NSWindowCollectionBehavior::NSWindowCollectionBehaviorStationary
            | NSWindowCollectionBehavior::NSWindowCollectionBehaviorFullScreenAuxiliary;
        
        ns_window.setCollectionBehavior_(behavior);
    }).map_err(|e| format!("Failed to set window level: {:?}", e))
}

/// 关闭截图窗口
#[tauri::command]
pub async fn close_draw_window(app: AppHandle) -> Result<(), String> {
    log::info!("API: close_draw_window called");

    if let Some(window) = app.get_webview_window("draw") {
        window
            .destroy()
            .map_err(|e| format!("Failed to destroy window: {}", e))?;
    }

    Ok(())
}
