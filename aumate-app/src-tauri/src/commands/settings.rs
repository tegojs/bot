// Settings Commands
use crate::state::AppState;
use aumate_core_domain::settings::Settings;
use aumate_core_shared::ApiError;
use tauri::{Emitter, State};

/// Get application settings
#[tauri::command]
pub async fn get_settings(state: State<'_, AppState>) -> Result<Settings, String> {
    log::info!("API: get_settings called");

    state.get_settings.execute().await.map_err(|e| {
        let api_error: ApiError = e.into();
        api_error.to_string()
    })
}

/// Save application settings
#[tauri::command]
pub async fn save_settings(
    state: State<'_, AppState>,
    app: tauri::AppHandle,
    settings: Settings,
) -> Result<(), String> {
    log::info!("API: save_settings called");

    state.save_settings.execute(settings.clone()).await.map_err(|e| {
        let api_error: ApiError = e.into();
        api_error.to_string()
    })?;

    // Emit settings-changed event to all windows
    let _ = app.emit("settings-changed", &settings);

    Ok(())
}
