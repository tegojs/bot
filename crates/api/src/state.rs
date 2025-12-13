// Application State 管理
use aumate_application::use_cases::{
    CaptureRegionUseCase, CaptureScreenUseCase, ScrollScreenshotUseCase, WindowManagementUseCase,
    clipboard::{
        ReadClipboardImageUseCase, ReadClipboardUseCase, WriteClipboardImageUseCase,
        WriteClipboardUseCase,
    },
    monitor::{GetCurrentMonitorUseCase, GetMonitorsUseCase},
};
use aumate_infrastructure::adapters::{
    ClipboardAdapter, HotkeyListenerAdapter, PageManagementAdapter, ScreenCaptureAdapter,
    UIAutomationAdapter,
};
use std::sync::Arc;

/// 应用状态
///
/// 存储所有 Use Cases，通过 Tauri State 管理
pub struct AppState {
    // Clipboard
    pub clipboard: Arc<ClipboardAdapter>,
    pub read_clipboard: Arc<ReadClipboardUseCase<ClipboardAdapter>>,
    pub write_clipboard: Arc<WriteClipboardUseCase<ClipboardAdapter>>,
    pub read_clipboard_image: Arc<ReadClipboardImageUseCase<ClipboardAdapter>>,
    pub write_clipboard_image: Arc<WriteClipboardImageUseCase<ClipboardAdapter>>,

    // Screenshot Use Cases
    pub capture_screen: Arc<CaptureScreenUseCase>,
    pub capture_region: Arc<CaptureRegionUseCase>,

    // Scroll Screenshot Use Case
    pub scroll_screenshot: Arc<ScrollScreenshotUseCase>,

    // Window Management Use Case
    pub window_management: Arc<WindowManagementUseCase>,

    // Monitor Use Cases
    pub get_monitors: Arc<GetMonitorsUseCase<ScreenCaptureAdapter>>,
    pub get_current_monitor: Arc<GetCurrentMonitorUseCase<ScreenCaptureAdapter>>,

    // UI Automation
    pub ui_automation: Arc<UIAutomationAdapter>,

    // Hotkey Management
    pub hotkey_listener: Arc<HotkeyListenerAdapter>,

    // Page Management
    pub page_management: Arc<PageManagementAdapter>,
}
