// Window Vibrancy Adapter - Infrastructure Layer
// Implements WindowVibrancyPort for Tauri windows

use async_trait::async_trait;
use aumate_core_shared::{InfrastructureError, WindowId};
use aumate_core_traits::{VibrancyEffect, WindowVibrancyPort};
use std::collections::HashMap;
use std::sync::Arc;
use tauri::WebviewWindow;
use tokio::sync::RwLock;

/// Window Vibrancy Adapter
///
/// 负责管理 Tauri 窗口的毛玻璃效果
pub struct WindowVibrancyAdapter {
    windows: Arc<RwLock<HashMap<WindowId, WebviewWindow>>>,
    /// 跟踪每个窗口是否启用了毛玻璃效果
    vibrancy_state: Arc<RwLock<HashMap<WindowId, bool>>>,
}

impl WindowVibrancyAdapter {
    pub fn new() -> Self {
        Self {
            windows: Arc::new(RwLock::new(HashMap::new())),
            vibrancy_state: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a window
    pub async fn register_window(&self, window_id: WindowId, window: WebviewWindow) {
        let mut windows = self.windows.write().await;
        windows.insert(window_id, window);
    }

    /// Get window by ID
    async fn get_window(&self, window_id: &WindowId) -> Result<WebviewWindow, InfrastructureError> {
        let windows = self.windows.read().await;
        windows.get(window_id).cloned().ok_or_else(|| {
            InfrastructureError::PlatformOperationFailed(format!(
                "Window not found: {:?}",
                window_id
            ))
        })
    }
}

impl Default for WindowVibrancyAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl WindowVibrancyPort for WindowVibrancyAdapter {
    async fn enable_vibrancy(
        &self,
        window_id: &WindowId,
        effect: VibrancyEffect,
    ) -> Result<(), InfrastructureError> {
        let window = self.get_window(window_id).await?;

        #[cfg(target_os = "windows")]
        {
            use tauri::utils::config::{Color, WindowEffectsConfig};
            use tauri::window::Effect;

            let tauri_effect = match effect {
                VibrancyEffect::Acrylic => Effect::Acrylic,
                VibrancyEffect::Mica => Effect::Mica,
                _ => Effect::Acrylic, // Default to Acrylic on Windows
            };

            let effects = WindowEffectsConfig {
                effects: vec![tauri_effect],
                state: None,
                radius: None,
                color: Some(Color(0, 0, 0, 50)),
            };

            window.set_effects(effects).map_err(|e| {
                InfrastructureError::PlatformOperationFailed(format!(
                    "Failed to enable vibrancy: {}",
                    e
                ))
            })?;
        }

        #[cfg(target_os = "macos")]
        {
            use tauri::utils::config::WindowEffectsConfig;
            use tauri::window::Effect;

            let tauri_effect = match effect {
                VibrancyEffect::HudWindow => Effect::HudWindow,
                VibrancyEffect::Sidebar => Effect::Sidebar,
                _ => Effect::HudWindow, // Default to HudWindow on macOS
            };

            let effects = WindowEffectsConfig {
                effects: vec![tauri_effect],
                state: None,
                radius: None,
                color: None,
            };

            window.set_effects(effects).map_err(|e| {
                InfrastructureError::PlatformOperationFailed(format!(
                    "Failed to enable vibrancy: {}",
                    e
                ))
            })?;
        }

        // Update state
        let mut state = self.vibrancy_state.write().await;
        state.insert(window_id.clone(), true);

        Ok(())
    }

    async fn disable_vibrancy(&self, window_id: &WindowId) -> Result<(), InfrastructureError> {
        let window = self.get_window(window_id).await?;

        window
            .set_effects(None::<tauri::utils::config::WindowEffectsConfig>)
            .map_err(|e| {
                InfrastructureError::PlatformOperationFailed(format!(
                    "Failed to disable vibrancy: {}",
                    e
                ))
            })?;

        // Update state
        let mut state = self.vibrancy_state.write().await;
        state.insert(window_id.clone(), false);

        Ok(())
    }

    async fn is_vibrancy_enabled(&self, window_id: &WindowId) -> Result<bool, InfrastructureError> {
        let state = self.vibrancy_state.read().await;
        Ok(state.get(window_id).copied().unwrap_or(false))
    }
}
