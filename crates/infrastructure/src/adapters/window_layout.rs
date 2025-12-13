// Window Layout Adapter - Infrastructure Layer
// Implements WindowLayoutPort for Tauri windows

use async_trait::async_trait;
use aumate_core_shared::{InfrastructureError, WindowId};
use aumate_core_traits::{MonitorInfo, WindowLayout, WindowLayoutPort};
use std::collections::HashMap;
use std::sync::Arc;
use tauri::{AppHandle, Manager, WebviewWindow};
use tokio::sync::RwLock;

/// Window Layout Adapter
pub struct WindowLayoutAdapter {
    app: Arc<RwLock<Option<AppHandle>>>,
    windows: Arc<RwLock<HashMap<WindowId, WebviewWindow>>>,
}

impl WindowLayoutAdapter {
    pub fn new() -> Self {
        Self {
            app: Arc::new(RwLock::new(None)),
            windows: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Initialize with AppHandle
    pub async fn init(&self, app: AppHandle) {
        let mut app_guard = self.app.write().await;
        *app_guard = Some(app);
    }

    /// Register a window
    pub async fn register_window(&self, window_id: WindowId, window: WebviewWindow) {
        let mut windows = self.windows.write().await;
        windows.insert(window_id, window);
    }

    /// Get window by ID
    async fn get_window(&self, window_id: &WindowId) -> Result<WebviewWindow, InfrastructureError> {
        let windows = self.windows.read().await;
        windows
            .get(window_id)
            .cloned()
            .ok_or_else(|| InfrastructureError::PlatformOperationFailed(format!("Window not found: {:?}", window_id)))
    }
}

impl Default for WindowLayoutAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl WindowLayoutPort for WindowLayoutAdapter {
    async fn get_window_layout(&self, window_id: &WindowId) -> Result<WindowLayout, InfrastructureError> {
        let window = self.get_window(window_id).await?;

        // Get monitor info for scale factor
        let monitor = window
            .current_monitor()
            .map_err(|e| InfrastructureError::PlatformOperationFailed(format!("Failed to get monitor: {}", e)))?
            .ok_or_else(|| InfrastructureError::PlatformOperationFailed("No monitor found".to_string()))?;

        let scale_factor = monitor.scale_factor();

        // Get window size (physical pixels) and convert to logical
        let size = window
            .inner_size()
            .map_err(|e| InfrastructureError::PlatformOperationFailed(format!("Failed to get size: {}", e)))?;
        let width = size.width as f64 / scale_factor;
        let height = size.height as f64 / scale_factor;

        // Get window position (physical pixels) and convert to logical
        let pos = window
            .outer_position()
            .map_err(|e| {
                InfrastructureError::PlatformOperationFailed(format!("Failed to get position: {}", e))
            })?;
        let x = pos.x as f64 / scale_factor;
        let y = pos.y as f64 / scale_factor;

        Ok(WindowLayout {
            width,
            height,
            x,
            y,
        })
    }

    async fn get_monitor_info(&self, window_id: &WindowId) -> Result<MonitorInfo, InfrastructureError> {
        let window = self.get_window(window_id).await?;

        let monitor = window
            .current_monitor()
            .map_err(|e| InfrastructureError::PlatformOperationFailed(format!("Failed to get monitor: {}", e)))?
            .ok_or_else(|| InfrastructureError::PlatformOperationFailed("No monitor found".to_string()))?;

        let size = monitor.size();
        let position = monitor.position();
        let scale_factor = monitor.scale_factor();

        Ok(MonitorInfo {
            width: size.width,
            height: size.height,
            position_x: position.x,
            position_y: position.y,
            scale_factor,
        })
    }

    async fn set_window_layout(
        &self,
        window_id: &WindowId,
        layout: WindowLayout,
    ) -> Result<(), InfrastructureError> {
        let window = self.get_window(window_id).await?;

        // Set size (using logical pixels)
        window
            .set_size(tauri::LogicalSize::new(layout.width, layout.height))
            .map_err(|e| InfrastructureError::PlatformOperationFailed(format!("Failed to set size: {}", e)))?;

        // Set position (using logical pixels)
        window
            .set_position(tauri::LogicalPosition::new(layout.x, layout.y))
            .map_err(|e| {
                InfrastructureError::PlatformOperationFailed(format!("Failed to set position: {}", e))
            })?;

        Ok(())
    }
}
