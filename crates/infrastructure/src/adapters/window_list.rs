// 窗口列表适配器
use async_trait::async_trait;
use aumate_core_shared::InfrastructureError;
use aumate_core_traits::{WindowListPort, window::WindowInfo};

/// 窗口列表适配器
///
/// 提供获取系统窗口列表的功能
pub struct WindowListAdapter {}

impl WindowListAdapter {
    pub fn new() -> Self {
        log::info!("Creating WindowListAdapter");
        Self {}
    }
}

impl Default for WindowListAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl WindowListPort for WindowListAdapter {
    async fn get_window_list(&self) -> Result<Vec<WindowInfo>, InfrastructureError> {
        log::info!("WindowListAdapter: getting window list");
        
        #[cfg(target_os = "macos")]
        {
            get_windows_macos().await
        }
        
        #[cfg(target_os = "windows")]
        {
            get_windows_windows().await
        }
        
        #[cfg(target_os = "linux")]
        {
            get_windows_linux().await
        }
    }
    
    async fn get_active_window(&self) -> Result<Option<WindowInfo>, InfrastructureError> {
        log::info!("WindowListAdapter: getting active window");
        
        #[cfg(target_os = "macos")]
        {
            get_active_window_macos().await
        }
        
        #[cfg(target_os = "windows")]
        {
            get_active_window_windows().await
        }
        
        #[cfg(target_os = "linux")]
        {
            get_active_window_linux().await
        }
    }
}

// ===== macOS 实现 =====
#[cfg(target_os = "macos")]
async fn get_windows_macos() -> Result<Vec<WindowInfo>, InfrastructureError> {
    use crate::platform::macos::window_list::get_all_windows;
    
    get_all_windows()
        .map_err(|e| InfrastructureError::PlatformOperationFailed(e))
}

#[cfg(target_os = "macos")]
async fn get_active_window_macos() -> Result<Option<WindowInfo>, InfrastructureError> {
    use active_win_pos_rs::get_active_window;
    use aumate_core_shared::Rectangle;
    
    match get_active_window() {
        Ok(window) => {
            let bounds = Rectangle::from_xywh(
                window.position.x as i32,
                window.position.y as i32,
                window.position.width as u32,
                window.position.height as u32,
            )
            .map_err(|e| InfrastructureError::PlatformOperationFailed(e.to_string()))?;
            
            // Parse window_id from String to u32
            let window_id_num = window.window_id.parse::<u32>().unwrap_or(0);
            
            let window_info = WindowInfo {
                id: window.window_id.clone(),
                window_id: window_id_num,
                title: window.title,
                app_name: window.app_name.clone(),
                process_name: window.app_name,
                process_path: window.process_path.to_string_lossy().to_string(),
                icon: None,
                bounds,
            };
            Ok(Some(window_info))
        }
        Err(_e) => {
            log::warn!("Failed to get active window");
            Ok(None)
        }
    }
}

// ===== Windows 实现 =====
#[cfg(target_os = "windows")]
async fn get_windows_windows() -> Result<Vec<WindowInfo>, InfrastructureError> {
    // TODO: 实现 Windows 窗口列表获取
    log::warn!("Windows window list not yet implemented");
    Ok(vec![])
}

#[cfg(target_os = "windows")]
async fn get_active_window_windows() -> Result<Option<WindowInfo>, InfrastructureError> {
    use active_win_pos_rs::get_active_window;
    use aumate_core_shared::Rectangle;
    
    match get_active_window() {
        Ok(window) => {
            let bounds = Rectangle::from_xywh(
                window.position.x as i32,
                window.position.y as i32,
                window.position.width as u32,
                window.position.height as u32,
            )
            .map_err(|e| InfrastructureError::PlatformOperationFailed(e.to_string()))?;
            
            // Parse window_id from String to u32
            let window_id_num = window.window_id.parse::<u32>().unwrap_or(0);
            
            let window_info = WindowInfo {
                id: window.window_id.clone(),
                window_id: window_id_num,
                title: window.title,
                app_name: window.app_name.clone(),
                process_name: window.app_name,
                process_path: window.process_path.to_string_lossy().to_string(),
                icon: None,
                bounds,
            };
            Ok(Some(window_info))
        }
        Err(_e) => {
            log::warn!("Failed to get active window");
            Ok(None)
        }
    }
}

// ===== Linux 实现 =====
#[cfg(target_os = "linux")]
async fn get_windows_linux() -> Result<Vec<WindowInfo>, InfrastructureError> {
    // TODO: 实现 Linux 窗口列表获取
    log::warn!("Linux window list not yet implemented");
    Ok(vec![])
}

#[cfg(target_os = "linux")]
async fn get_active_window_linux() -> Result<Option<WindowInfo>, InfrastructureError> {
    // TODO: 实现 Linux 活动窗口获取
    log::warn!("Linux active window not yet implemented");
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_adapter_creation() {
        let adapter = WindowListAdapter::new();
        drop(adapter);
    }
}
