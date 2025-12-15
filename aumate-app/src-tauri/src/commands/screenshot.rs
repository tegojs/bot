// 截图相关 Tauri Commands
use crate::state::AppState;
use aumate_application::dto::{CaptureRegionRequest, CaptureResponse, CaptureScreenRequest};
use aumate_core_shared::{ApiError, DomainError, MonitorId, Rectangle};
use image::DynamicImage;
use serde::Serialize;
use tauri::{AppHandle, State};

/// 捕获当前监视器
#[tauri::command]
pub async fn capture_current_monitor(
    state: State<'_, AppState>,
    format: String,
    quality: Option<u8>,
    hdr_correction: Option<bool>,
) -> Result<CaptureResponse, String> {
    log::info!("API: capture_current_monitor called");

    let request = CaptureScreenRequest {
        monitor_id: None,
        format,
        quality,
        hdr_correction: hdr_correction.unwrap_or(false),
    };

    state.capture_screen.execute(request).await.map_err(|e| {
        let api_error: ApiError = e.into();
        api_error.to_string()
    })
}

/// 捕获指定监视器
#[tauri::command]
pub async fn capture_monitor(
    state: State<'_, AppState>,
    monitor_id: String,
    format: String,
    quality: Option<u8>,
    hdr_correction: Option<bool>,
) -> Result<CaptureResponse, String> {
    log::info!("API: capture_monitor called, monitor_id={}", monitor_id);

    let request = CaptureScreenRequest {
        monitor_id: Some(MonitorId::new(monitor_id.parse().unwrap_or(0))),
        format,
        quality,
        hdr_correction: hdr_correction.unwrap_or(false),
    };

    state.capture_screen.execute(request).await.map_err(|e| {
        let api_error: ApiError = e.into();
        api_error.to_string()
    })
}

/// 捕获屏幕区域
#[tauri::command]
pub async fn capture_region(
    state: State<'_, AppState>,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    monitor_id: Option<String>,
    format: String,
    quality: Option<u8>,
) -> Result<CaptureResponse, String> {
    log::info!("API: capture_region called, region=({},{},{}x{})", x, y, width, height);

    let region = Rectangle::new(x, y, x + width as i32, y + height as i32)
        .map_err(|e: DomainError| e.to_string())?;

    let request = CaptureRegionRequest {
        region,
        monitor_id: monitor_id.map(|id| MonitorId::new(id.parse().unwrap_or(0))),
        format,
        quality,
    };

    state.capture_region.execute(request).await.map_err(|e| {
        let api_error: ApiError = e.into();
        api_error.to_string()
    })
}

// ============= 截图编辑器专用命令 =============

#[derive(Debug, Clone, Serialize)]
pub struct ElementRect {
    pub min_x: i32,
    pub min_y: i32,
    pub max_x: i32,
    pub max_y: i32,
}

impl ElementRect {
    pub fn scale(&self, factor: f32) -> Self {
        Self {
            min_x: (self.min_x as f32 * factor) as i32,
            min_y: (self.min_y as f32 * factor) as i32,
            max_x: (self.max_x as f32 * factor) as i32,
            max_y: (self.max_y as f32 * factor) as i32,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct WindowElement {
    pub element_rect: ElementRect,
    pub window_id: u32,
}

/// 捕获所有显示器的截图（用于截图编辑器）
#[tauri::command]
pub async fn capture_all_monitors(
    _app: AppHandle,
    window: tauri::WebviewWindow,
) -> Result<Vec<u8>, String> {
    log::info!("API: capture_all_monitors called");

    // 检查是否有显示器（Monitor 不是 Send，所以不能跨越 await）
    let monitor_count = xcap::Monitor::all()
        .map_err(|e| format!("Failed to get monitors: {}", e))?
        .len();

    if monitor_count == 0 {
        return Err("No monitors found".to_string());
    }

    // 暂时隐藏截图窗口
    let _ = window.hide();

    // 等待窗口隐藏
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    // 在 await 之后获取 monitors（避免跨越 await 点）
    let monitors = xcap::Monitor::all().map_err(|e| format!("Failed to get monitors after await: {}", e))?;

    // 先截取所有显示器，获取实际的物理像素尺寸
    let mut captures: Vec<(i32, i32, image::RgbaImage)> = Vec::new();
    let mut first_reported_width = 0u32;
    
    for monitor in &monitors {
        let x = monitor.x().unwrap_or(0);
        let y = monitor.y().unwrap_or(0);
        let width = monitor.width().unwrap_or(0);
        
        if first_reported_width == 0 {
            first_reported_width = width;
        }

        match monitor.capture_image() {
            Ok(monitor_image) => {
                log::info!(
                    "Monitor capture: reported size {}x{}, actual image {}x{}",
                    width,
                    monitor.height().unwrap_or(0),
                    monitor_image.width(),
                    monitor_image.height()
                );
                captures.push((x, y, monitor_image));
            }
            Err(e) => {
                log::error!("Failed to capture monitor at ({}, {}): {}", x, y, e);
            }
        }
    }

    // 重新显示窗口
    let _ = window.show();

    if captures.is_empty() {
        return Err("Failed to capture any monitor".to_string());
    }

    // 计算缩放比例（物理像素 / CSS像素）
    let scale_factor = if let Some((_, _, img)) = captures.first() {
        let reported_width = first_reported_width as f64;
        img.width() as f64 / reported_width.max(1.0)
    } else {
        1.0
    };
    log::info!("Detected scale factor: {}", scale_factor);

    // 使用物理像素计算边界框
    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;
    let mut max_x = i32::MIN;
    let mut max_y = i32::MIN;

    for (i, (x, y, img)) in captures.iter().enumerate() {
        // 坐标也需要按比例缩放
        let scaled_x = (*x as f64 * scale_factor) as i32;
        let scaled_y = (*y as f64 * scale_factor) as i32;

        min_x = min_x.min(scaled_x);
        min_y = min_y.min(scaled_y);
        max_x = max_x.max(scaled_x + img.width() as i32);
        max_y = max_y.max(scaled_y + img.height() as i32);

        log::info!(
            "Monitor {}: CSS pos ({}, {}), scaled pos ({}, {}), size {}x{}",
            i, x, y, scaled_x, scaled_y, img.width(), img.height()
        );
    }

    let total_width = (max_x - min_x) as u32;
    let total_height = (max_y - min_y) as u32;

    log::info!(
        "Creating combined image: {}x{} (physical pixels)",
        total_width,
        total_height
    );

    // 创建物理像素大小的画布
    let mut combined_image = image::RgbaImage::new(total_width, total_height);

    // 合并所有截图
    for (x, y, monitor_image) in captures {
        let scaled_x = (x as f64 * scale_factor) as i32;
        let scaled_y = (y as f64 * scale_factor) as i32;
        let offset_x = (scaled_x - min_x) as i64;
        let offset_y = (scaled_y - min_y) as i64;

        image::imageops::overlay(
            &mut combined_image,
            &monitor_image,
            offset_x,
            offset_y,
        );
    }

    // 编码为 PNG
    let mut buffer = Vec::new();
    DynamicImage::ImageRgba8(combined_image)
        .write_to(
            &mut std::io::Cursor::new(&mut buffer),
            image::ImageFormat::Png,
        )
        .map_err(|e| format!("Failed to encode image: {}", e))?;

    log::info!("Screenshot captured: {} bytes", buffer.len());

    Ok(buffer)
}

/// 获取智能窗口识别的窗口列表（用于框选时的智能识别）
#[tauri::command]
pub async fn get_screenshot_window_elements() -> Result<Vec<WindowElement>, String> {
    log::info!("API: get_screenshot_window_elements called");

    let windows = xcap::Window::all().map_err(|e| format!("Failed to get windows: {}", e))?;
    log::info!("Total windows found: {}", windows.len());

    #[cfg(target_os = "macos")]
    let scale_factor = {
        // macOS 下窗口基于逻辑像素，需要转换为物理像素
        use core_graphics::display::CGDisplay;
        let display_id = CGDisplay::main().id;
        match CGDisplay::new(display_id).display_mode() {
            Some(mode) => mode.pixel_height() as f32 / mode.height() as f32,
            None => {
                log::warn!("Failed to get display mode, using scale factor 1.0");
                1.0
            }
        }
    };

    #[cfg(not(target_os = "macos"))]
    let scale_factor = 1.0f32;

    log::info!("Using scale factor: {}", scale_factor);

    let mut window_elements = Vec::new();

    for window in windows {
        // 跳过最小化的窗口
        if window.is_minimized().unwrap_or(false) {
            continue;
        }

        let window_id = window.id().unwrap_or(0);
        let title = window.title().unwrap_or_default();
        
        // 获取窗口位置和尺寸（用于过滤不可见窗口）
        let x = window.x().unwrap_or(0);
        let y = window.y().unwrap_or(0);
        let width = window.width().unwrap_or(0) as i32;
        let height = window.height().unwrap_or(0) as i32;

        // 过滤一些系统窗口
        #[cfg(target_os = "macos")]
        {
            if title == "Notification Center"
                || title == "Dock"
                || title.starts_with("Item-")
                || title.is_empty()
            {
                continue;
            }
            
            // 过滤特殊系统窗口（如 Menubar）
            let app_name = window.app_name().unwrap_or_default();
            if title == "Menubar" && app_name == "Window Server" {
                continue;
            }
            
            // 过滤 Cursor（Window Server 的窗口）
            if title == "Cursor" && app_name == "Window Server" {
                continue;
            }
        }

        #[cfg(target_os = "windows")]
        {
            if title == "Shell Handwriting Canvas" {
                continue;
            }
        }

        // 过滤太小的窗口（可能是系统元素或隐藏窗口）
        if width < 20 || height < 20 {
            continue;
        }

        // 过滤无效尺寸的窗口
        if width <= 0 || height <= 0 {
            continue;
        }
        
        // 过滤位置异常的窗口（远离屏幕的窗口通常是不可见的）
        // 假设屏幕最大 10000x10000，超出范围的窗口可能是隐藏的
        if x.abs() > 10000 || y.abs() > 10000 {
            continue;
        }

        let rect = ElementRect {
            min_x: x,
            min_y: y,
            max_x: x + width,
            max_y: y + height,
        };

        let scaled_rect = rect.scale(scale_factor);
        log::debug!(
            "Window '{}' (id={}): ({}, {}) {}x{} -> scaled: ({}, {}) {}x{}",
            title,
            window_id,
            x,
            y,
            width,
            height,
            scaled_rect.min_x,
            scaled_rect.min_y,
            scaled_rect.max_x - scaled_rect.min_x,
            scaled_rect.max_y - scaled_rect.min_y
        );

        // 返回原始坐标（CSS像素/逻辑像素），前端会根据需要处理 devicePixelRatio
        window_elements.push(WindowElement {
            element_rect: rect,
            window_id,
        });
    }

    log::info!("Found {} screenshot window elements", window_elements.len());

    Ok(window_elements)
}

// Note: get_mouse_position 可以由前端 JavaScript 通过 DOM 事件跟踪鼠标位置
// 如果后端需要，可以使用 enigo 或其他库实现

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capture_request_creation() {
        let request = CaptureScreenRequest {
            monitor_id: None,
            format: "png".to_string(),
            quality: Some(95),
            hdr_correction: false,
        };
        assert_eq!(request.format, "png");
        assert_eq!(request.quality, Some(95));
    }
}
