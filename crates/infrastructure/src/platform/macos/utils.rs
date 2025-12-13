/// macOS 平台工具函数
///
/// **迁移**: 从 app-os/src/utils/macos.rs

pub fn get_focused_window() -> Option<()> {
    None
}

pub fn switch_always_on_top() {
    log::warn!("[platform::macos::utils::switch_always_on_top] not implemented");
}

pub fn set_draw_window_style(#[allow(unused_variables)] window: tauri::Window) {
    // macOS 无需实现
}

pub fn create_admin_auto_start_task() -> Result<(), String> {
    Ok(())
}

pub fn delete_admin_auto_start_task() -> Result<(), String> {
    Ok(())
}

pub fn restart_with_admin() -> Result<(), String> {
    Ok(())
}

pub fn is_admin() -> bool {
    false
}
