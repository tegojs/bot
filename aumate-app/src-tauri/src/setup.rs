// Application Setup - 依赖注入和初始化
use crate::state::AppState;
use aumate_application::use_cases::{
    CaptureRegionUseCase, CaptureScreenUseCase, ScrollScreenshotUseCase, WindowManagementUseCase,
    GetWindowElementsUseCase,
    clipboard::{
        ReadClipboardImageUseCase, ReadClipboardUseCase, WriteClipboardImageUseCase,
        WriteClipboardUseCase,
    },
    monitor::{GetCurrentMonitorUseCase, GetMonitorsUseCase},
    settings::{GetSettingsUseCase, SaveSettingsUseCase},
};
use aumate_infrastructure::adapters::{
    ClipboardAdapter, FileSystemSettingsAdapter, HotkeyListenerAdapter, ImageProcessingAdapter,
    PageManagementAdapter, ScreenCaptureAdapter, ScrollCaptureAdapter, UIAutomationAdapter,
    WindowManagementAdapter, WindowListAdapter,
};
use std::sync::Arc;

/// 设置应用程序
///
/// 创建所有 Adapters 和 Use Cases，并注入依赖
pub fn setup_application() -> AppState {
    log::info!("Setting up application...");

    // 1. 创建 Infrastructure Adapters
    log::info!("Creating infrastructure adapters...");

    let clipboard = Arc::new(ClipboardAdapter::new());
    let screen_capture = Arc::new(ScreenCaptureAdapter::new());
    let image_processing = Arc::new(ImageProcessingAdapter::new());
    let scroll_capture = Arc::new(ScrollCaptureAdapter::new());
    let window_management = Arc::new(WindowManagementAdapter::new());
    let window_list = Arc::new(WindowListAdapter::new());
    let ui_automation = Arc::new(UIAutomationAdapter::new());
    let hotkey_listener = Arc::new(HotkeyListenerAdapter::new());
    let page_management = Arc::new(PageManagementAdapter::new());
    let settings_storage = Arc::new(FileSystemSettingsAdapter::new());

    // 2. 创建 Use Cases
    log::info!("Creating use cases...");

    // Clipboard Use Cases
    let read_clipboard = Arc::new(ReadClipboardUseCase::new(clipboard.clone()));
    let write_clipboard = Arc::new(WriteClipboardUseCase::new(clipboard.clone()));
    let read_clipboard_image = Arc::new(ReadClipboardImageUseCase::new(clipboard.clone()));
    let write_clipboard_image = Arc::new(WriteClipboardImageUseCase::new(clipboard.clone()));

    let capture_screen =
        Arc::new(CaptureScreenUseCase::new(screen_capture.clone(), image_processing.clone()));

    let capture_region =
        Arc::new(CaptureRegionUseCase::new(screen_capture.clone(), image_processing.clone()));

    let scroll_screenshot = Arc::new(ScrollScreenshotUseCase::new(scroll_capture));

    let window_management_use_case = Arc::new(WindowManagementUseCase::new(window_management));
    
    // Window List Use Cases
    let get_window_elements = Arc::new(GetWindowElementsUseCase::new(window_list.clone()));

    let get_monitors = Arc::new(GetMonitorsUseCase::new(screen_capture.clone()));
    let get_current_monitor = Arc::new(GetCurrentMonitorUseCase::new(screen_capture.clone()));

    // Settings Use Cases
    let get_settings = Arc::new(GetSettingsUseCase::new(settings_storage.clone()));
    let save_settings = Arc::new(SaveSettingsUseCase::new(settings_storage.clone()));

    // 3. 创建 AppState
    log::info!("Application setup complete!");

    AppState {
        clipboard,
        read_clipboard,
        write_clipboard,
        read_clipboard_image,
        write_clipboard_image,
        capture_screen,
        capture_region,
        scroll_screenshot,
        window_management: window_management_use_case,
        window_list,
        get_window_elements,
        get_monitors,
        get_current_monitor,
        ui_automation,
        hotkey_listener,
        page_management,
        settings_storage,
        get_settings,
        save_settings,
    }
}
