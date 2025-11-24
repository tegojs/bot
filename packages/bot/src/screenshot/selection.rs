// Interactive region selection with window snapping

use super::types::*;
use std::sync::{Arc, Mutex};

/// Start interactive capture with overlay window
pub async fn start_interactive_capture(
    options: InteractiveCaptureOptions,
    _state: Arc<Mutex<Option<SelectionState>>>,
) -> Result<ScreenshotResult, String> {
    // Use the overlay module for interactive capture
    crate::screenshot::overlay::run_interactive_overlay(options).await
}

/// Detect windows on screen for snapping
pub fn detect_windows() -> Result<Vec<WindowSnapInfo>, String> {
    use active_win_pos_rs::get_active_window;

    // Get active window info
    let active_window =
        get_active_window().map_err(|_| "Failed to get active window".to_string())?;

    let window_info = WindowSnapInfo {
        window_id: active_window.process_path.to_string_lossy().to_string(),
        title: active_window.title,
        region: ScreenRegion {
            x: active_window.position.x as i32,
            y: active_window.position.y as i32,
            width: active_window.position.width as u32,
            height: active_window.position.height as u32,
        },
        snap_edge: String::new(),
    };

    Ok(vec![window_info])
}

/// Check if selection should snap to any window
pub fn check_window_snap(
    selection: &ScreenRegion,
    windows: &[WindowSnapInfo],
    threshold: u32,
) -> Option<WindowSnapInfo> {
    for window in windows {
        if should_snap_to_window(selection, &window.region, threshold) {
            let snap_edge = determine_snap_edges(selection, &window.region, threshold);
            let mut snapped = window.clone();
            snapped.snap_edge = snap_edge;
            return Some(snapped);
        }
    }
    None
}

/// Determine if selection should snap to window
fn should_snap_to_window(selection: &ScreenRegion, window: &ScreenRegion, threshold: u32) -> bool {
    let threshold = threshold as i32;

    // Check if any edge is within threshold
    let left_close = (selection.x - window.x).abs() <= threshold;
    let right_close = ((selection.x + selection.width as i32) - (window.x + window.width as i32))
        .abs()
        <= threshold;
    let top_close = (selection.y - window.y).abs() <= threshold;
    let bottom_close =
        ((selection.y + selection.height as i32) - (window.y + window.height as i32)).abs()
            <= threshold;

    left_close || right_close || top_close || bottom_close
}

/// Determine which edges are snapping
fn determine_snap_edges(selection: &ScreenRegion, window: &ScreenRegion, threshold: u32) -> String {
    let threshold = threshold as i32;
    let mut edges = Vec::new();

    if (selection.x - window.x).abs() <= threshold {
        edges.push("left");
    }
    if ((selection.x + selection.width as i32) - (window.x + window.width as i32)).abs()
        <= threshold
    {
        edges.push("right");
    }
    if (selection.y - window.y).abs() <= threshold {
        edges.push("top");
    }
    if ((selection.y + selection.height as i32) - (window.y + window.height as i32)).abs()
        <= threshold
    {
        edges.push("bottom");
    }

    if edges.len() == 4 { "all".to_string() } else { edges.join(",") }
}

/// Snap selection to window
pub fn snap_to_window(selection: &mut ScreenRegion, window: &ScreenRegion, snap_edges: &str) {
    for edge in snap_edges.split(',') {
        match edge {
            "left" => selection.x = window.x,
            "right" => selection.x = window.x + window.width as i32 - selection.width as i32,
            "top" => selection.y = window.y,
            "bottom" => selection.y = window.y + window.height as i32 - selection.height as i32,
            "all" => {
                selection.x = window.x;
                selection.y = window.y;
                selection.width = window.width;
                selection.height = window.height;
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_snap_to_window() {
        let window = ScreenRegion::new(100, 100, 400, 300);
        let selection_near = ScreenRegion::new(95, 105, 200, 150);
        let selection_far = ScreenRegion::new(50, 50, 200, 150);

        assert!(should_snap_to_window(&selection_near, &window, 10));
        assert!(!should_snap_to_window(&selection_far, &window, 10));
    }

    #[test]
    fn test_snap_to_window() {
        let window = ScreenRegion::new(100, 100, 400, 300);
        let mut selection = ScreenRegion::new(95, 105, 200, 150);

        snap_to_window(&mut selection, &window, "left,top");

        assert_eq!(selection.x, 100);
        assert_eq!(selection.y, 100);
    }
}
