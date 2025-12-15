use async_trait::async_trait;
use aumate_core_shared::InfrastructureError;
use aumate_core_traits::GlobalShortcutPort;
use std::sync::Arc;
use tauri::AppHandle;

/// 全局快捷键适配器
///
/// 封装 Tauri 的全局快捷键功能
///
/// **依赖**: tauri-plugin-global-shortcut
pub struct GlobalShortcutAdapter {
    app_handle: Arc<AppHandle>,
}

impl GlobalShortcutAdapter {
    pub fn new(app_handle: AppHandle) -> Self {
        Self {
            app_handle: Arc::new(app_handle),
        }
    }
}

#[async_trait]
impl GlobalShortcutPort for GlobalShortcutAdapter {
    async fn register(&self, shortcut: &str) -> Result<(), InfrastructureError> {
        #[cfg(desktop)]
        {
            use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut};

            let parsed_shortcut: Shortcut = shortcut
                .parse()
                .map_err(|e| InfrastructureError::InvalidInput(format!("Invalid shortcut format '{}': {}", shortcut, e)))?;

            self.app_handle
                .global_shortcut()
                .register(parsed_shortcut)
                .map_err(|e| {
                    InfrastructureError::PlatformOperationFailed(format!(
                        "Failed to register shortcut '{}': {}",
                        shortcut, e
                    ))
                })?;

            log::info!("Successfully registered global shortcut: {}", shortcut);
            Ok(())
        }

        #[cfg(not(desktop))]
        {
            Err(InfrastructureError::PlatformOperationFailed(
                "Global shortcuts are not supported on this platform".to_string(),
            ))
        }
    }

    async fn unregister(&self, shortcut: &str) -> Result<(), InfrastructureError> {
        #[cfg(desktop)]
        {
            use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut};

            let parsed_shortcut: Shortcut = shortcut
                .parse()
                .map_err(|e| InfrastructureError::InvalidInput(format!("Invalid shortcut format '{}': {}", shortcut, e)))?;

            self.app_handle
                .global_shortcut()
                .unregister(parsed_shortcut)
                .map_err(|e| {
                    InfrastructureError::PlatformOperationFailed(format!(
                        "Failed to unregister shortcut '{}': {}",
                        shortcut, e
                    ))
                })?;

            log::info!("Successfully unregistered global shortcut: {}", shortcut);
            Ok(())
        }

        #[cfg(not(desktop))]
        {
            Err(InfrastructureError::PlatformOperationFailed(
                "Global shortcuts are not supported on this platform".to_string(),
            ))
        }
    }

    async fn is_available(&self, shortcut: &str) -> Result<bool, InfrastructureError> {
        #[cfg(desktop)]
        {
            use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut};

            let parsed_shortcut: Shortcut = shortcut
                .parse()
                .map_err(|e| InfrastructureError::InvalidInput(format!("Invalid shortcut format '{}': {}", shortcut, e)))?;

            // Try to register, then immediately unregister if successful
            match self.app_handle.global_shortcut().register(parsed_shortcut) {
                Ok(_) => {
                    // Successfully registered, so it's available
                    let _ = self.app_handle.global_shortcut().unregister(parsed_shortcut);
                    Ok(true)
                }
                Err(_) => {
                    // Failed to register, so it's not available
                    Ok(false)
                }
            }
        }

        #[cfg(not(desktop))]
        {
            Ok(false)
        }
    }
}

