// 滚动截图相关 Tauri Commands
use crate::state::AppState;
use aumate_application::dto::{ScrollCaptureResponse, StartScrollCaptureRequest};
use aumate_core_shared::{ApiError, DomainError, Rectangle};
use tauri::State;

/// 开始滚动截图
#[tauri::command]
pub async fn start_scroll_capture(
    state: State<'_, AppState>,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    direction: String,
    max_frames: Option<usize>,
) -> Result<ScrollCaptureResponse, String> {
    log::info!(
        "API: start_scroll_capture called, region=({},{},{}x{}), direction={}",
        x,
        y,
        width,
        height,
        direction
    );

    let region = Rectangle::new(x, y, x + width as i32, y + height as i32)
        .map_err(|e: DomainError| e.to_string())?;

    let request = StartScrollCaptureRequest { monitor_id: None, region, direction, max_frames };

    state.scroll_screenshot.execute(request).await.map_err(|e| {
        let api_error: ApiError = e.into();
        api_error.to_string()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scroll_capture_request() {
        let region = Rectangle::new(0, 0, 800, 600).unwrap();
        let request = StartScrollCaptureRequest {
            monitor_id: None,
            region,
            direction: "vertical".to_string(),
            max_frames: Some(100),
        };
        assert_eq!(request.direction, "vertical");
        assert_eq!(request.max_frames, Some(100));
    }
}
