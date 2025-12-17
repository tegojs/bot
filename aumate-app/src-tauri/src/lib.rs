use tauri::{
    Emitter, Manager,
    image::Image,
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
};

// Import commands and state management
mod commands;
mod setup;
mod shortcut_parser;
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
    // 初始化日志，默认显示 info 级别
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            // Setup application state with AppHandle for global shortcut management
            let app_state = setup_application(app.handle().clone());
            app.manage(app_state);

            // Note: Window vibrancy is now configured via windowEffects in tauri.conf.json
            // Runtime vibrancy control is handled by the set_window_vibrancy command

            // Get settings window for centering
            let settings_window = app.get_webview_window("settings").unwrap();

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
                let _ =
                    settings_window.set_position(tauri::LogicalPosition::new(target_x, target_y));
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
                        if let Some(window) = app.get_webview_window("commandpalette") {
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

            // Register global shortcuts from settings
            #[cfg(desktop)]
            {
                use shortcut_parser::parse_shortcut;
                use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

                // 从设置中读取快捷键配置
                let shortcuts_config = {
                    let app_state = app.state::<state::AppState>();
                    // 使用 block_on 同步获取设置
                    match tauri::async_runtime::block_on(app_state.get_settings.execute()) {
                        Ok(settings) => settings.shortcuts,
                        Err(e) => {
                            log::error!("Failed to load shortcuts settings: {}", e);
                            // 使用默认快捷键
                            aumate_core_domain::settings::ShortcutSettings::default()
                        }
                    }
                };

                log::info!("Registering shortcuts from settings...");
                log::info!("  toggle_palette: {}", shortcuts_config.toggle_palette);
                log::info!("  screenshot: {}", shortcuts_config.screenshot);
                log::info!("  element_scan: {}", shortcuts_config.element_scan);

                // 解析快捷键字符串
                let toggle_palette_shortcut = match parse_shortcut(&shortcuts_config.toggle_palette)
                {
                    Ok(s) => s,
                    Err(e) => {
                        log::error!(
                            "Failed to parse toggle_palette shortcut '{}': {}",
                            shortcuts_config.toggle_palette,
                            e
                        );
                        parse_shortcut("F3").unwrap() // fallback
                    }
                };

                let screenshot_shortcut = match parse_shortcut(&shortcuts_config.screenshot) {
                    Ok(s) => s,
                    Err(e) => {
                        log::error!(
                            "Failed to parse screenshot shortcut '{}': {}",
                            shortcuts_config.screenshot,
                            e
                        );
                        parse_shortcut("Ctrl+4").unwrap() // fallback
                    }
                };

                let element_scan_shortcut = match parse_shortcut(&shortcuts_config.element_scan) {
                    Ok(s) => s,
                    Err(e) => {
                        log::error!(
                            "Failed to parse element_scan shortcut '{}': {}",
                            shortcuts_config.element_scan,
                            e
                        );
                        parse_shortcut("Ctrl+5").unwrap() // fallback
                    }
                };

                // 注册全局快捷键处理器
                app.handle().plugin(
                    tauri_plugin_global_shortcut::Builder::new()
                        .with_handler(move |app_handle, hotkey, event| {
                            if event.state == ShortcutState::Pressed {
                                if hotkey == &toggle_palette_shortcut {
                                    if let Some(window) =
                                        app_handle.get_webview_window("commandpalette")
                                    {
                                        toggle_window(&window);
                                    }
                                } else if hotkey == &screenshot_shortcut {
                                    let app_handle_clone = app_handle.clone();
                                    tauri::async_runtime::spawn(async move {
                                        if let Err(e) =
                                            commands::create_draw_window(app_handle_clone).await
                                        {
                                            log::error!("Failed to create draw window: {}", e);
                                        }
                                    });
                                } else if hotkey == &element_scan_shortcut {
                                    // Ctrl+5 作为切换键：如果已打开则关闭，否则打开
                                    if let Some(window) =
                                        app_handle.get_webview_window("elementscan")
                                    {
                                        if let Ok(is_visible) = window.is_visible() {
                                            if is_visible {
                                                log::info!(
                                                    "Element scan window is visible, hiding it"
                                                );
                                                let _ = window.hide();
                                            } else {
                                                log::info!(
                                                    "Element scan window is hidden, showing it"
                                                );
                                                let app_handle_clone = app_handle.clone();
                                                tauri::async_runtime::spawn(async move {
                                                    if let Err(e) = commands::start_element_scan(
                                                        app_handle_clone,
                                                    )
                                                    .await
                                                    {
                                                        log::error!(
                                                            "Failed to start element scan: {}",
                                                            e
                                                        );
                                                    }
                                                });
                                            }
                                        }
                                    }
                                }
                            }
                        })
                        .build(),
                )?;

                // 注册快捷键
                if let Err(e) = app.global_shortcut().register(toggle_palette_shortcut) {
                    log::warn!(
                        "Failed to register toggle_palette hotkey '{}': {}",
                        shortcuts_config.toggle_palette,
                        e
                    );
                }

                if let Err(e) = app.global_shortcut().register(screenshot_shortcut) {
                    log::warn!(
                        "Failed to register screenshot hotkey '{}': {}",
                        shortcuts_config.screenshot,
                        e
                    );
                }

                if let Err(e) = app.global_shortcut().register(element_scan_shortcut) {
                    log::warn!(
                        "Failed to register element_scan hotkey '{}': {}",
                        shortcuts_config.element_scan,
                        e
                    );
                }

                log::info!("Global shortcuts registered successfully");
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // UI control commands
            show_command_palette,
            hide_command_palette,
            toggle_command_palette,
            start_screenshot,
            start_element_scan,
            // Settings commands
            get_settings,
            save_settings,
            // Screenshot commands
            capture_current_monitor,
            capture_monitor,
            capture_region,
            capture_all_monitors,
            get_screenshot_window_elements,
            // Draw window commands
            create_draw_window,
            close_draw_window,
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
            write_clipboard_image_png,
            // Window management commands
            create_window,
            drag_window,
            resize_window,
            pin_window,
            unpin_window,
            close_window,
            get_window_elements,
            switch_to_window,
            close_desktop_window,
            resize_and_center,
            animate_resize_and_center,
            set_window_vibrancy,
            // Permissions commands
            commands::permissions::check_permissions,
            commands::permissions::request_screen_recording_permission,
            commands::permissions::request_accessibility_permission,
            commands::permissions::request_microphone_permission,
            // UI automation commands
            get_element_from_position,
            init_ui_elements,
            // Global Shortcut commands
            register_global_shortcut,
            unregister_global_shortcut,
            check_global_shortcut_availability,
            // Element Scanner commands
            scan_screen_elements,
            trigger_element_action,
            // Page management commands
            add_page,
            remove_page,
            // Scroll screenshot commands
            start_scroll_capture,
            // Log commands
            frontend_log,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
