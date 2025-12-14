// 前端日志转发到 Rust 终端

#[tauri::command]
pub fn frontend_log(level: String, message: String) {
    match level.as_str() {
        "error" => log::error!("[Frontend] {}", message),
        "warn" => log::warn!("[Frontend] {}", message),
        "info" => log::info!("[Frontend] {}", message),
        "debug" => log::debug!("[Frontend] {}", message),
        _ => log::info!("[Frontend] {}", message),
    }
}
