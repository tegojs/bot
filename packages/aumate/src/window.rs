//! Window management module
//!
//! Provides window information and management for desktop automation.
//!
//! Note: This implementation uses active-win-pos-rs which provides
//! get_active_window() functionality. Getting a list of all windows
//! requires platform-specific implementations.

use crate::error::{AumateError, Result};
use active_win_pos_rs::{ActiveWindow, get_active_window};

/// Window information structure
#[derive(Debug, Clone)]
pub struct WindowInfo {
    /// Window title
    pub title: String,
    /// Process ID
    pub process_id: u32,
    /// Process path/name
    pub process_path: String,
    /// Window position X
    pub x: f64,
    /// Window position Y
    pub y: f64,
    /// Window width
    pub width: f64,
    /// Window height
    pub height: f64,
    /// Window ID (platform-specific)
    pub window_id: String,
}

impl WindowInfo {
    /// Create WindowInfo from active_win_pos_rs::ActiveWindow
    fn from_active_window(window: ActiveWindow) -> Self {
        Self {
            title: window.title,
            process_id: window.process_id as u32,
            process_path: window.process_path.to_string_lossy().to_string(),
            x: window.position.x,
            y: window.position.y,
            width: window.position.width,
            height: window.position.height,
            window_id: window.window_id.to_string(),
        }
    }

    /// Get the process name from the path
    pub fn process_name(&self) -> &str {
        self.process_path.rsplit(std::path::MAIN_SEPARATOR).next().unwrap_or(&self.process_path)
    }
}

/// Get the currently active (focused) window
pub fn get_active_window_info() -> Result<WindowInfo> {
    let active_window = get_active_window()
        .map_err(|_| AumateError::Window("Failed to get active window".to_string()))?;

    Ok(WindowInfo::from_active_window(active_window))
}

/// Get a list of all visible windows
/// Note: Currently only returns the active window due to API limitations
pub fn get_all_windows() -> Result<Vec<WindowInfo>> {
    let active_window = get_active_window()
        .map_err(|_| AumateError::Window("Failed to get active window".to_string()))?;

    Ok(vec![WindowInfo::from_active_window(active_window)])
}

/// Find windows by title (case-insensitive partial match)
/// Note: Currently only searches the active window due to API limitations
pub fn find_windows_by_title(search_title: &str) -> Result<Vec<WindowInfo>> {
    let active_window = get_active_window()
        .map_err(|_| AumateError::Window("Failed to get active window".to_string()))?;

    let search_lower = search_title.to_lowercase();
    if active_window.title.to_lowercase().contains(&search_lower) {
        Ok(vec![WindowInfo::from_active_window(active_window)])
    } else {
        Ok(vec![])
    }
}

/// Find windows by process name (case-insensitive partial match)
/// Note: Currently only searches the active window due to API limitations
pub fn find_windows_by_process(process_name: &str) -> Result<Vec<WindowInfo>> {
    let active_window = get_active_window()
        .map_err(|_| AumateError::Window("Failed to get active window".to_string()))?;

    let process_lower = process_name.to_lowercase();
    let process_path_str = active_window.process_path.to_string_lossy().to_lowercase();

    if process_path_str.contains(&process_lower) {
        Ok(vec![WindowInfo::from_active_window(active_window)])
    } else {
        Ok(vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_active_window() {
        // This test requires a GUI environment
        let result = get_active_window_info();
        match result {
            Ok(window) => {
                println!("Active window: {}", window.title);
            }
            Err(e) => {
                println!("No active window found: {}", e);
            }
        }
    }

    #[test]
    fn test_get_all_windows() {
        let result = get_all_windows();
        match result {
            Ok(windows) => {
                println!("Found {} windows", windows.len());
                for window in windows.iter().take(5) {
                    println!("  - {}: {}", window.process_name(), window.title);
                }
            }
            Err(e) => {
                println!("Failed to get windows: {}", e);
            }
        }
    }
}
