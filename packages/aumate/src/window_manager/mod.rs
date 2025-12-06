//! Window Manager module
//!
//! Provides window positioning functionality with a command palette UI.
//!
//! This module is only available on macOS with the `window_manager` feature enabled.

pub mod actions;
pub mod config;
pub mod controller;
pub mod hotkey;
#[cfg(target_os = "macos")]
pub mod macos;
pub mod palette;

pub use actions::{CurrentBounds, ScreenBounds, TargetBounds, WINDOW_ACTIONS, WindowAction};
pub use config::{HotkeyConfig, Modifier, WindowManagerConfig};
pub use controller::WindowManagerFeature;
pub use hotkey::WindowManagerHotkeyManager;
pub use palette::{CommandPalette, PaletteResult, PaletteState};

#[cfg(target_os = "macos")]
pub use macos::{
    get_frontmost_app_pid, get_frontmost_app_pid_excluding, get_window_bounds,
    is_accessibility_trusted, set_window_frame, set_window_position, set_window_size,
};

use crate::error::Result;

/// Get the data directory for window manager config
pub fn get_window_manager_data_dir() -> Result<std::path::PathBuf> {
    config::get_window_manager_data_dir()
}

/// Execute a window action on a target process
#[cfg(target_os = "macos")]
pub fn execute_window_action(action_id: &str, target_pid: i32) -> Result<()> {
    use crate::error::AumateError;

    // Get screen bounds from xcap
    let monitors = xcap::Monitor::all()
        .map_err(|e| AumateError::Other(format!("Failed to get monitors: {}", e)))?;

    let primary =
        monitors.first().ok_or_else(|| AumateError::Other("No monitors found".to_string()))?;

    let screen_bounds = ScreenBounds {
        x: primary
            .x()
            .map_err(|e| AumateError::Other(format!("Failed to get monitor x: {}", e)))?,
        y: primary
            .y()
            .map_err(|e| AumateError::Other(format!("Failed to get monitor y: {}", e)))?,
        width: primary
            .width()
            .map_err(|e| AumateError::Other(format!("Failed to get monitor width: {}", e)))?,
        height: primary
            .height()
            .map_err(|e| AumateError::Other(format!("Failed to get monitor height: {}", e)))?,
    };

    // Get current window bounds (for center action)
    let current_bounds = get_window_bounds(target_pid).ok().map(|b| CurrentBounds {
        x: b.x as i32,
        y: b.y as i32,
        width: b.width as u32,
        height: b.height as u32,
    });

    // Calculate target bounds
    let action = WINDOW_ACTIONS
        .iter()
        .find(|a| a.id == action_id)
        .ok_or_else(|| AumateError::Other(format!("Unknown action: {}", action_id)))?;

    let target = action.calculate(screen_bounds, current_bounds);

    // Apply the window frame
    log::info!(
        "Executing action '{}' on pid {}: x={}, y={}, w={}, h={}",
        action_id,
        target_pid,
        target.x,
        target.y,
        target.width,
        target.height
    );

    set_window_frame(
        target_pid,
        target.x as f64,
        target.y as f64,
        target.width as f64,
        target.height as f64,
    )
}

#[cfg(not(target_os = "macos"))]
pub fn execute_window_action(_action_id: &str, _target_pid: i32) -> Result<()> {
    Err(crate::error::AumateError::Other("Window Manager is only supported on macOS".to_string()))
}
