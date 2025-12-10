use image::DynamicImage;
use xcap::Monitor;

use super::types::{CaptureRegion, ElementRect};

/// Get the monitor at the given point (usually mouse position)
pub fn get_monitor_at_point(x: i32, y: i32) -> Result<Monitor, String> {
    Monitor::from_point(x, y).map_err(|e| format!("Failed to get monitor at point: {}", e))
}

/// Get all available monitors
pub fn get_all_monitors() -> Result<Vec<Monitor>, String> {
    Monitor::all().map_err(|e| format!("Failed to get monitors: {}", e))
}

/// Capture the entire screen of a specific monitor
pub fn capture_monitor(monitor: &Monitor) -> Result<DynamicImage, String> {
    let image = monitor
        .capture_image()
        .map_err(|e| format!("Failed to capture monitor: {}", e))?;
    Ok(DynamicImage::ImageRgba8(image))
}

/// Capture a specific region of a monitor
pub fn capture_monitor_region(
    monitor: &Monitor,
    region: &CaptureRegion,
) -> Result<DynamicImage, String> {
    let image = monitor
        .capture_image()
        .map_err(|e| format!("Failed to capture monitor: {}", e))?;

    let dynamic_image = DynamicImage::ImageRgba8(image);

    // Crop to the specified region
    let cropped = dynamic_image.crop_imm(region.x, region.y, region.width, region.height);

    Ok(cropped)
}

/// Capture all monitors and combine into a single image
pub fn capture_all_monitors() -> Result<(DynamicImage, ElementRect), String> {
    let monitors = get_all_monitors()?;

    if monitors.is_empty() {
        return Err("No monitors found".to_string());
    }

    // Calculate the bounding box of all monitors
    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;
    let mut max_x = i32::MIN;
    let mut max_y = i32::MIN;

    for monitor in &monitors {
        let x = monitor.x();
        let y = monitor.y();
        let width = monitor.width() as i32;
        let height = monitor.height() as i32;

        min_x = min_x.min(x);
        min_y = min_y.min(y);
        max_x = max_x.max(x + width);
        max_y = max_y.max(y + height);
    }

    let total_width = (max_x - min_x) as u32;
    let total_height = (max_y - min_y) as u32;

    // Create a new image to hold all monitors
    let mut combined = image::RgbaImage::new(total_width, total_height);

    // Capture each monitor and place it in the combined image
    for monitor in &monitors {
        let monitor_image = monitor
            .capture_image()
            .map_err(|e| format!("Failed to capture monitor: {}", e))?;

        let x = monitor.x();
        let y = monitor.y();

        // Calculate offset in the combined image
        let offset_x = (x - min_x) as u32;
        let offset_y = (y - min_y) as u32;

        // Copy pixels
        for (px, py, pixel) in monitor_image.enumerate_pixels() {
            let target_x = offset_x + px;
            let target_y = offset_y + py;
            if target_x < total_width && target_y < total_height {
                combined.put_pixel(target_x, target_y, *pixel);
            }
        }
    }

    let bounds = ElementRect::new(min_x, min_y, max_x, max_y);

    Ok((DynamicImage::ImageRgba8(combined), bounds))
}

/// Capture the monitor where the mouse is currently located
pub fn capture_current_monitor(mouse_x: i32, mouse_y: i32) -> Result<DynamicImage, String> {
    let monitor = get_monitor_at_point(mouse_x, mouse_y)?;
    capture_monitor(&monitor)
}

/// Capture a specific region across all monitors
pub fn capture_region(region: &ElementRect) -> Result<DynamicImage, String> {
    let (full_image, bounds) = capture_all_monitors()?;

    // Calculate the crop region relative to the combined image
    let crop_x = (region.min_x - bounds.min_x).max(0) as u32;
    let crop_y = (region.min_y - bounds.min_y).max(0) as u32;
    let crop_width = region.width() as u32;
    let crop_height = region.height() as u32;

    let cropped = full_image.crop_imm(crop_x, crop_y, crop_width, crop_height);

    Ok(cropped)
}

/// Get information about the monitor at a specific point
pub fn get_monitor_info(x: i32, y: i32) -> Result<MonitorInfo, String> {
    let monitor = get_monitor_at_point(x, y)?;

    Ok(MonitorInfo {
        x: monitor.x(),
        y: monitor.y(),
        width: monitor.width(),
        height: monitor.height(),
        name: monitor.name().to_string(),
        scale_factor: monitor.scale_factor(),
    })
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct MonitorInfo {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub name: String,
    pub scale_factor: f32,
}
