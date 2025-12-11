use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

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

// Screenshot and UI automation modules
pub mod screenshot;
pub mod ui_automation;

use screenshot::types::{ElementRect, ImageFormat, WindowElement};

// Global state for UI automation
struct AppState {
    ui_elements: Mutex<ui_automation::UIElements>,
}

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
    pub open_settings: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedSettings {
    pub debug_mode: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpressionPolishingSettings {
    pub api_url: String,
    pub api_key: String,
    pub model: String,
    pub system_prompt: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIDialogueSettings {
    pub api_url: String,
    pub api_key: String,
    pub model: String,
    pub system_prompt: String,
    pub max_history_messages: i32,
}

impl Default for AIDialogueSettings {
    fn default() -> Self {
        Self {
            api_url: "https://api.openai.com/v1".to_string(),
            api_key: String::new(),
            model: "gpt-4".to_string(),
            system_prompt: "You are a helpful assistant.".to_string(),
            max_history_messages: 20,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnabledModes {
    pub search: bool,
    pub polish: bool,
    pub dialogue: bool,
    #[serde(default = "default_true")]
    pub switcher: bool,
}

fn default_true() -> bool {
    true
}

impl Default for EnabledModes {
    fn default() -> Self {
        Self {
            search: true,
            polish: true,
            dialogue: true,
            switcher: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenshotSettings {
    pub save_folder: String,
    pub filename_pattern: String,
    pub image_format: String,
    pub auto_copy_clipboard: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub general: GeneralSettings,
    pub shortcuts: ShortcutSettings,
    pub advanced: AdvancedSettings,
    pub expression_polishing: ExpressionPolishingSettings,
    pub screenshot: ScreenshotSettings,
    #[serde(default)]
    pub ai_dialogue: AIDialogueSettings,
    #[serde(default)]
    pub enabled_modes: EnabledModes,
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
            shortcuts: ShortcutSettings {
                toggle_palette: "F3".to_string(),
                open_settings: "Ctrl+,".to_string(),
            },
            advanced: AdvancedSettings { debug_mode: false },
            expression_polishing: ExpressionPolishingSettings {
                api_url: "https://api.openai.com/v1".to_string(),
                api_key: String::new(),
                model: "gpt-4".to_string(),
                system_prompt: "You are an expression polishing assistant. When given text:\n1. Provide a polished, improved version of the expression\n2. Explain the key adjustments you made\n\nFormat your response as:\n**Polished:**\n[improved text]\n\n**Adjustments:**\n[bullet points explaining changes]".to_string(),
            },
            screenshot: ScreenshotSettings {
                save_folder: String::new(),
                filename_pattern: "screenshot_%Y%m%d_%H%M%S".to_string(),
                image_format: "png".to_string(),
                auto_copy_clipboard: true,
            },
            ai_dialogue: AIDialogueSettings::default(),
            enabled_modes: EnabledModes::default(),
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

// Command to save settings and notify other windows
#[tauri::command]
async fn save_settings(app: tauri::AppHandle, settings: Settings) -> Result<(), String> {
    ensure_settings_dir().map_err(|e| e.to_string())?;
    let path = get_settings_path();
    let content = serde_json::to_string_pretty(&settings).map_err(|e| e.to_string())?;
    fs::write(&path, content).map_err(|e| e.to_string())?;

    // Emit settings-changed event to all windows so they can reload
    let _ = app.emit("settings-changed", &settings);

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

/// Start screenshot mode by showing the screenshot window
#[tauri::command]
async fn start_screenshot(app: tauri::AppHandle) -> Result<(), String> {
    // Hide the main window first
    if let Some(main_window) = app.get_webview_window("main") {
        let _ = main_window.hide();
    }

    // Show and focus the screenshot window
    if let Some(screenshot_window) = app.get_webview_window("screenshot") {
        screenshot_window.show().map_err(|e| e.to_string())?;
        screenshot_window.set_focus().map_err(|e| e.to_string())?;
    }

    Ok(())
}

// ============= Screenshot Commands =============

/// Capture all monitors and return as base64 encoded image
#[tauri::command]
async fn capture_all_monitors(format: Option<ImageFormat>) -> Result<String, String> {
    let format = format.unwrap_or(ImageFormat::Png);
    let (image, _bounds) = screenshot::capture::capture_all_monitors()?;
    let bytes = screenshot::encode::encode_image(&image, format)?;
    Ok(base64::Engine::encode(
        &base64::engine::general_purpose::STANDARD,
        &bytes,
    ))
}

/// Capture the monitor at the given mouse position
#[tauri::command]
async fn capture_current_monitor(
    mouse_x: i32,
    mouse_y: i32,
    format: Option<ImageFormat>,
) -> Result<String, String> {
    let format = format.unwrap_or(ImageFormat::Png);
    let image = screenshot::capture::capture_current_monitor(mouse_x, mouse_y)?;
    let bytes = screenshot::encode::encode_image(&image, format)?;
    Ok(base64::Engine::encode(
        &base64::engine::general_purpose::STANDARD,
        &bytes,
    ))
}

/// Capture a specific region
#[tauri::command]
async fn capture_region(region: ElementRect, format: Option<ImageFormat>) -> Result<String, String> {
    let format = format.unwrap_or(ImageFormat::Png);
    let image = screenshot::capture::capture_region(&region)?;
    let bytes = screenshot::encode::encode_image(&image, format)?;
    Ok(base64::Engine::encode(
        &base64::engine::general_purpose::STANDARD,
        &bytes,
    ))
}

/// Get all visible windows
#[tauri::command]
async fn get_window_elements() -> Result<Vec<WindowElement>, String> {
    ui_automation::get_all_windows()
}

/// Get the window at a specific point
#[tauri::command]
async fn get_window_at_point(x: i32, y: i32) -> Result<Option<WindowElement>, String> {
    ui_automation::get_window_at_point(x, y)
}

/// Get the UI element at a specific point
#[tauri::command]
async fn get_element_at_point(
    state: tauri::State<'_, AppState>,
    x: i32,
    y: i32,
) -> Result<Option<ElementRect>, String> {
    let ui_elements = state.ui_elements.lock().map_err(|e| e.to_string())?;
    ui_elements.get_element_at_point(x, y)
}

/// Initialize UI automation
#[tauri::command]
async fn init_ui_automation(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let mut ui_elements = state.ui_elements.lock().map_err(|e| e.to_string())?;
    ui_elements.init()
}

/// Save screenshot to file
#[tauri::command]
async fn save_screenshot(
    image_data: String,
    file_path: String,
    format: Option<ImageFormat>,
) -> Result<(), String> {
    let format = format.unwrap_or(ImageFormat::Png);
    let bytes = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, &image_data)
        .map_err(|e| format!("Failed to decode base64: {}", e))?;
    let image = screenshot::encode::decode_image(&bytes)?;
    let path = std::path::Path::new(&file_path);
    screenshot::encode::save_image_to_file(&image, path, format).await
}

/// Get monitor info at a specific point
#[tauri::command]
async fn get_monitor_info(x: i32, y: i32) -> Result<screenshot::capture::MonitorInfo, String> {
    screenshot::capture::get_monitor_info(x, y)
}

/// Switch to a window by its ID
#[tauri::command]
async fn switch_to_window(window_id: u32) -> Result<(), String> {
    ui_automation::switch_to_window(window_id)
}

/// Close a window by its ID
#[tauri::command]
async fn close_window(window_id: u32) -> Result<(), String> {
    ui_automation::close_window(window_id)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            ui_elements: Mutex::new(ui_automation::UIElements::new()),
        })
        .setup(|app| {
            // Apply vibrancy to main window
            let main_window = app.get_webview_window("main").unwrap();
            #[cfg(target_os = "windows")]
            {
                apply_mica(&main_window, Some(true)).expect("Failed to apply mica effect to main");
            }
            #[cfg(target_os = "macos")]
            {
                apply_vibrancy(&main_window, NSVisualEffectMaterial::HudWindow, None, None)
                    .expect("Failed to apply vibrancy to main");
            }

            // Apply vibrancy to settings window
            let settings_window = app.get_webview_window("settings").unwrap();
            #[cfg(target_os = "windows")]
            {
                apply_mica(&settings_window, Some(true)).expect("Failed to apply mica effect to settings");
            }
            #[cfg(target_os = "macos")]
            {
                apply_vibrancy(&settings_window, NSVisualEffectMaterial::HudWindow, None, None)
                    .expect("Failed to apply vibrancy to settings");
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
            save_settings,
            // Screenshot commands
            start_screenshot,
            capture_all_monitors,
            capture_current_monitor,
            capture_region,
            get_window_elements,
            get_window_at_point,
            get_element_at_point,
            init_ui_automation,
            save_screenshot,
            get_monitor_info,
            // Window switcher commands
            switch_to_window,
            close_window
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
