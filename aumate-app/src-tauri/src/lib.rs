use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use tauri::{
    image::Image,
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Emitter, Manager,
};

#[cfg(target_os = "windows")]
use window_vibrancy::apply_mica;

#[cfg(target_os = "macos")]
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};

// Settings schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralSettings {
    pub follow_system_appearance: bool,
    pub open_at_login: bool,
    pub show_in_system_tray: bool,
    pub hotkey: String,
    pub window_mode: String, // "compact" or "expanded"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortcutSettings {
    pub toggle_palette: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedSettings {
    pub debug_mode: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub general: GeneralSettings,
    pub shortcuts: ShortcutSettings,
    pub advanced: AdvancedSettings,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            general: GeneralSettings {
                follow_system_appearance: true,
                open_at_login: false,
                show_in_system_tray: true,
                hotkey: "F3".to_string(),
                window_mode: "compact".to_string(),
            },
            shortcuts: ShortcutSettings { toggle_palette: "F3".to_string() },
            advanced: AdvancedSettings { debug_mode: false },
        }
    }
}

// Get settings file path
fn get_settings_path() -> PathBuf {
    let home = dirs::home_dir().expect("Could not find home directory");
    home.join(".aumate").join("settings.json")
}

// Ensure settings directory exists
fn ensure_settings_dir() -> std::io::Result<()> {
    let path = get_settings_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    Ok(())
}

// Helper function to toggle window visibility
fn toggle_window(window: &tauri::WebviewWindow) {
    let is_visible = window.is_visible().unwrap_or(false);
    if is_visible {
        let _ = window.hide();
    } else {
        let _ = window.center();
        let _ = window.show();
        let _ = window.set_focus();
    }
}

// Helper function to show settings window
fn show_settings_window(app: &tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("settings") {
        let _ = window.show();
        let _ = window.center();
        let _ = window.set_focus();
    }
}

// Command to get settings
#[tauri::command]
async fn get_settings() -> Result<Settings, String> {
    let path = get_settings_path();
    if path.exists() {
        let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
        let settings: Settings = serde_json::from_str(&content).unwrap_or_default();
        Ok(settings)
    } else {
        Ok(Settings::default())
    }
}

// Command to save settings
#[tauri::command]
async fn save_settings(settings: Settings) -> Result<(), String> {
    ensure_settings_dir().map_err(|e| e.to_string())?;
    let path = get_settings_path();
    let content = serde_json::to_string_pretty(&settings).map_err(|e| e.to_string())?;
    fs::write(&path, content).map_err(|e| e.to_string())?;
    Ok(())
}

// Command to show the command palette window
#[tauri::command]
async fn show_command_palette(window: tauri::Window) -> Result<(), String> {
    window.center().map_err(|e| e.to_string())?;
    window.show().map_err(|e| e.to_string())?;
    window.set_focus().map_err(|e| e.to_string())?;
    Ok(())
}

// Command to hide the command palette window
#[tauri::command]
async fn hide_command_palette(window: tauri::Window) -> Result<(), String> {
    window.hide().map_err(|e| e.to_string())?;
    Ok(())
}

// Command to toggle the command palette window visibility
#[tauri::command]
async fn toggle_command_palette(window: tauri::Window) -> Result<(), String> {
    if window.is_visible().map_err(|e| e.to_string())? {
        window.hide().map_err(|e| e.to_string())?;
    } else {
        window.center().map_err(|e| e.to_string())?;
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();

            // Apply vibrancy effect based on platform
            #[cfg(target_os = "windows")]
            {
                // Apply mica effect on Windows 11 (includes rounded corners)
                apply_mica(&window, Some(true)).expect("Failed to apply mica effect");
            }

            #[cfg(target_os = "macos")]
            {
                // Apply vibrancy effect on macOS
                apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, None)
                    .expect("Failed to apply vibrancy effect");
            }

            // Create system tray menu
            let about = MenuItem::with_id(app, "about", "About Aumate", true, None::<&str>)?;
            let settings = MenuItem::with_id(app, "settings", "Settings", true, None::<&str>)?;
            let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&about, &settings, &quit])?;

            // Create system tray
            let _tray = TrayIconBuilder::new()
                .icon(
                    Image::from_path("icons/32x32.png")
                        .unwrap_or_else(|_| app.default_window_icon().unwrap().clone()),
                )
                .menu(&menu)
                .show_menu_on_left_click(false)
                .tooltip("Aumate - Press F3 to toggle")
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            toggle_window(&window);
                        }
                    }
                })
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "about" => {
                        // Open settings window and navigate to About section
                        show_settings_window(app);
                        if let Some(window) = app.get_webview_window("settings") {
                            let _ = window.emit("navigate", "about");
                        }
                    }
                    "settings" => {
                        // Open settings window
                        show_settings_window(app);
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .build(app)?;

            // Register global shortcut
            #[cfg(desktop)]
            {
                use tauri_plugin_global_shortcut::{
                    Code, GlobalShortcutExt, Shortcut, ShortcutState,
                };

                // Create the shortcut: F3 to toggle the command palette
                let shortcut = Shortcut::new(None, Code::F3);

                // Register the plugin with a handler
                app.handle().plugin(
                    tauri_plugin_global_shortcut::Builder::new()
                        .with_handler(move |app_handle, hotkey, event| {
                            if event.state == ShortcutState::Pressed && hotkey == &shortcut {
                                if let Some(window) = app_handle.get_webview_window("main") {
                                    toggle_window(&window);
                                }
                            }
                        })
                        .build(),
                )?;

                // Register the shortcut
                app.global_shortcut().register(shortcut)?;
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            show_command_palette,
            hide_command_palette,
            toggle_command_palette,
            get_settings,
            save_settings
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
