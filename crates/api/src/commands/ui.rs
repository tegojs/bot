// UI 元素识别相关 Tauri Commands
use aumate_core_shared::Rectangle;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIElement {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

/// 从位置获取 UI 元素
///
/// 注意：此功能需要 macOS Accessibility 权限
/// 当前返回空列表，因为完整的 UI 元素遍历需要 Accessibility API 的完整实现
#[tauri::command]
pub async fn get_element_from_position(x: i32, y: i32) -> Result<Vec<UIElement>, String> {
    log::info!("API: get_element_from_position called, position=({}, {})", x, y);

    #[cfg(target_os = "macos")]
    {
        // macOS 需要 accessibility 权限
        if !macos_accessibility_client::accessibility::application_is_trusted() {
            return Err("Accessibility permission not granted".to_string());
        }

        // 返回空列表（完整实现需要深度集成 macOS Accessibility API）
        log::info!(
            "get_element_from_position: returning empty list (Accessibility API integration required)"
        );
        Ok(vec![])
    }

    #[cfg(not(target_os = "macos"))]
    {
        Err("Platform not supported".to_string())
    }
}

/// 初始化 UI 元素缓存
#[tauri::command]
pub async fn init_ui_elements() -> Result<(), String> {
    log::info!("API: init_ui_elements called");

    // macOS 需要 accessibility 权限
    #[cfg(target_os = "macos")]
    {
        if !macos_accessibility_client::accessibility::application_is_trusted() {
            return Err("Accessibility permission not granted".to_string());
        }
    }

    Ok(())
}
