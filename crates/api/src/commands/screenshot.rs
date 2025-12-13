// 截图相关 Tauri Commands
use crate::state::AppState;
use aumate_application::dto::{CaptureRegionRequest, CaptureResponse, CaptureScreenRequest};
use aumate_core_shared::{ApiError, DomainError, MonitorId, Rectangle};
use tauri::State;

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
