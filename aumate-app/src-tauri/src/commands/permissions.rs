// Permissions Commands
// Commands for checking and requesting system permissions

use serde::{Deserialize, Serialize};

/// 权限状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionStatus {
    /// 录屏与系统录音权限
    pub screen_recording: bool,
    /// 辅助功能权限
    pub accessibility: bool,
    /// 麦克风权限
    pub microphone: bool,
}

/// 检查所有权限状态
#[tauri::command]
pub async fn check_permissions() -> Result<PermissionStatus, String> {
    log::info!("API: check_permissions called");

    let status = PermissionStatus {
        screen_recording: check_screen_recording_permission(),
        accessibility: check_accessibility_permission(),
        microphone: check_microphone_permission(),
    };

    log::debug!("Permission status: {:?}", status);
    Ok(status)
}

/// 请求录屏权限
#[tauri::command]
pub async fn request_screen_recording_permission() -> Result<(), String> {
    log::info!("API: request_screen_recording_permission called");

    #[cfg(target_os = "macos")]
    {
        // 在 macOS 上，尝试截图会自动触发权限请求
        std::process::Command::new("screencapture")
            .arg("-x")
            .arg("-T")
            .arg("0")
            .output()
            .map_err(|e| format!("Failed to request screen recording permission: {}", e))?;
    }

    #[cfg(not(target_os = "macos"))]
    {
        // 其他平台通常不需要特殊请求
        log::warn!("Screen recording permission request is not needed on this platform");
    }

    Ok(())
}

/// 请求辅助功能权限
#[tauri::command]
pub async fn request_accessibility_permission() -> Result<(), String> {
    log::info!("API: request_accessibility_permission called");

    #[cfg(target_os = "macos")]
    {
        // 打开系统偏好设置的辅助功能面板
        std::process::Command::new("open")
            .arg("x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility")
            .spawn()
            .map_err(|e| format!("Failed to open accessibility settings: {}", e))?;
    }

    #[cfg(not(target_os = "macos"))]
    {
        log::warn!("Accessibility permission request is not needed on this platform");
    }

    Ok(())
}

/// 请求麦克风权限
#[tauri::command]
pub async fn request_microphone_permission() -> Result<(), String> {
    log::info!("API: request_microphone_permission called");

    #[cfg(target_os = "macos")]
    {
        // 打开系统偏好设置的麦克风面板
        std::process::Command::new("open")
            .arg("x-apple.systempreferences:com.apple.preference.security?Privacy_Microphone")
            .spawn()
            .map_err(|e| format!("Failed to open microphone settings: {}", e))?;
    }

    #[cfg(not(target_os = "macos"))]
    {
        log::warn!("Microphone permission request is not needed on this platform");
    }

    Ok(())
}

/// 检查录屏权限
fn check_screen_recording_permission() -> bool {
    #[cfg(target_os = "macos")]
    {
        // 在 macOS 上，尝试截取屏幕来检查权限
        // 简化实现：假设已授权，实际需要通过系统 API 检查
        // TODO: 实现真实的权限检查
        true
    }

    #[cfg(not(target_os = "macos"))]
    {
        // 其他平台默认有权限
        true
    }
}

/// 检查辅助功能权限
fn check_accessibility_permission() -> bool {
    #[cfg(target_os = "macos")]
    {
        macos_accessibility_client::accessibility::application_is_trusted()
    }

    #[cfg(not(target_os = "macos"))]
    {
        // 其他平台默认有权限
        true
    }
}

/// 检查麦克风权限
fn check_microphone_permission() -> bool {
    #[cfg(target_os = "macos")]
    {
        // 在 macOS 上，使用 AVCaptureDevice 来检查权限
        // 这里简化处理，假设有权限
        // 完整实现需要使用 AVFoundation framework
        true
    }

    #[cfg(not(target_os = "macos"))]
    {
        // 其他平台默认有权限
        true
    }
}

