use tauri::{
    Emitter, Manager,
    image::Image,
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
};

#[cfg(target_os = "windows")]
use window_vibrancy::apply_acrylic;

#[cfg(target_os = "macos")]
use window_vibrancy::{NSVisualEffectMaterial, apply_vibrancy};

// Import commands and state management
mod commands;
mod setup;
mod state;

use commands::*;
use setup::setup_application;

// Helper function to center window precisely
fn center_window_precise(window: &tauri::WebviewWindow) {
    if let Ok(Some(monitor)) = window.current_monitor() {
        let scale_factor = monitor.scale_factor();
        let monitor_size = monitor.size();
        let monitor_pos = monitor.position();
        
        // 转换为逻辑像素
        let screen_width = monitor_size.width as f64 / scale_factor;
        let screen_height = monitor_size.height as f64 / scale_factor;
        let monitor_x = monitor_pos.x as f64 / scale_factor;
        let monitor_y = monitor_pos.y as f64 / scale_factor;
        
        // 获取窗口当前尺寸（物理像素）并转换为逻辑像素
        if let Ok(window_size) = window.inner_size() {
            let window_width = window_size.width as f64 / scale_factor;
            let window_height = window_size.height as f64 / scale_factor;
            
            // 计算居中位置
            let target_x = monitor_x + (screen_width - window_width) / 2.0;
            let target_y = monitor_y + (screen_height - window_height) / 2.0;
            
            // 设置位置（使用 LogicalPosition）
            let _ = window.set_position(tauri::LogicalPosition::new(target_x, target_y));
        }
    }
}

// Helper function to toggle window visibility
fn toggle_window(window: &tauri::WebviewWindow) {
    let is_visible = window.is_visible().unwrap_or(false);
    if is_visible {
        let _ = window.hide();
    } else {
        center_window_precise(window);
        let _ = window.show();
        let _ = window.set_focus();
    }
}

// Helper function to show settings window
fn show_settings_window(app: &tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("settings") {
        center_window_precise(&window);
        let _ = window.show();
        let _ = window.set_focus();
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(setup_application()) // Use DDD architecture AppState
        .setup(|app| {
            // Apply vibrancy to main window
            let main_window = app.get_webview_window("main").unwrap();
            #[cfg(target_os = "windows")]
            {
                apply_acrylic(&main_window, Some((0, 0, 0, 50)))
                    .expect("Failed to apply mica effect to main");
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
                apply_acrylic(&settings_window, Some((0, 0, 0, 50)))
                    .expect("Failed to apply mica effect to settings");
            }
            #[cfg(target_os = "macos")]
            {
                apply_vibrancy(&settings_window, NSVisualEffectMaterial::HudWindow, None, None)
                    .expect("Failed to apply vibrancy to settings");
            }

            // 确保窗口正确居中
            if let Ok(Some(monitor)) = settings_window.current_monitor() {
                let scale_factor = monitor.scale_factor();
                let monitor_size = monitor.size();
                let monitor_pos = monitor.position();
                
                // 转换为逻辑像素
                let screen_width = monitor_size.width as f64 / scale_factor;
                let screen_height = monitor_size.height as f64 / scale_factor;
                let monitor_x = monitor_pos.x as f64 / scale_factor;
                let monitor_y = monitor_pos.y as f64 / scale_factor;
                
                // 窗口尺寸（从配置读取）
                let window_width = 900.0;
                let window_height = 600.0;
                
                // 计算居中位置
                let target_x = monitor_x + (screen_width - window_width) / 2.0;
                let target_y = monitor_y + (screen_height - window_height) / 2.0;
                
                // 设置位置
                let _ = settings_window.set_position(tauri::LogicalPosition::new(target_x, target_y));
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
                        show_settings_window(app);
                        if let Some(window) = app.get_webview_window("settings") {
                            let _ = window.emit("navigate", "about");
                        }
                    }
                    "settings" => {
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

                let shortcut = Shortcut::new(None, Code::F3);

                app.handle().plugin(
                    tauri_plugin_global_shortcut::Builder::new()
                        .with_handler(move |app_handle, hotkey, event| {
                            if event.state == ShortcutState::Pressed
                                && hotkey == &shortcut
                                && let Some(window) = app_handle.get_webview_window("main")
                            {
                                toggle_window(&window);
                            }
                        })
                        .build(),
                )?;

                app.global_shortcut().register(shortcut)?;
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // UI control commands
            show_command_palette,
            hide_command_palette,
            toggle_command_palette,
            start_screenshot,
            // Settings commands
            get_settings,
            save_settings,
            // Screenshot commands
            capture_current_monitor,
            capture_monitor,
            capture_region,
            // Monitor commands
            get_monitors,
            get_current_monitor,
            // Clipboard commands
            read_clipboard,
            write_clipboard,
            clear_clipboard,
            get_clipboard_types,
            read_clipboard_image,
            write_clipboard_image,
            // Window management commands
            create_window,
            drag_window,
            resize_window,
            pin_window,
            unpin_window,
            close_window,
            get_window_elements,
            resize_and_center,
            animate_resize_and_center,
            // UI automation commands
            get_element_from_position,
            init_ui_elements,
            // Hotkey commands
            listen_key_start,
            listen_key_stop,
            listen_mouse_start,
            listen_mouse_stop,
            // Page management commands
            add_page,
            remove_page,
            // Scroll screenshot commands
            start_scroll_capture,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
