// Application State 管理
use aumate_application::use_cases::{
    CaptureRegionUseCase, CaptureScreenUseCase, CheckGlobalShortcutAvailabilityUseCase,
    ClickElementUseCase, CloseDesktopWindowUseCase, FocusElementUseCase, GetWindowElementsUseCase,
    RegisterGlobalShortcutUseCase, ScanElementsUseCase, ScrollScreenshotUseCase,
    SetWindowVibrancyUseCase, SwitchToWindowUseCase, UnregisterGlobalShortcutUseCase,
    WindowManagementUseCase,
    clipboard::{
        ReadClipboardImageUseCase, ReadClipboardUseCase, WriteClipboardImageUseCase,
        WriteClipboardUseCase,
    },
    monitor::{GetCurrentMonitorUseCase, GetMonitorsUseCase},
    settings::{GetSettingsUseCase, SaveSettingsUseCase},
};
use aumate_infrastructure::adapters::{
    ClipboardAdapter, ElementScannerAdapter, FileSystemSettingsAdapter, GlobalShortcutAdapter,
    HotkeyListenerAdapter, PageManagementAdapter, ScreenCaptureAdapter, UIAutomationAdapter,
    WindowListAdapter, WindowVibrancyAdapter,
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

    // Window List
    pub window_list: Arc<WindowListAdapter>,
    pub get_window_elements: Arc<GetWindowElementsUseCase>,
    pub switch_to_window: Arc<SwitchToWindowUseCase>,
    pub close_desktop_window: Arc<CloseDesktopWindowUseCase>,

    // Window Layout
    pub window_layout: Arc<aumate_infrastructure::WindowLayoutAdapter>,
    pub resize_and_center: Arc<aumate_application::ResizeAndCenterUseCase>,
    pub animate_resize_and_center: Arc<aumate_application::AnimateResizeAndCenterUseCase>,

    // Window Vibrancy
    pub window_vibrancy: Arc<WindowVibrancyAdapter>,
    pub set_window_vibrancy: Arc<SetWindowVibrancyUseCase>,

    // Monitor Use Cases
    pub get_monitors: Arc<GetMonitorsUseCase<ScreenCaptureAdapter>>,
    pub get_current_monitor: Arc<GetCurrentMonitorUseCase<ScreenCaptureAdapter>>,

    // UI Automation
    pub ui_automation: Arc<UIAutomationAdapter>,

    // Hotkey Management
    pub hotkey_listener: Arc<HotkeyListenerAdapter>,

    // Page Management
    pub page_management: Arc<PageManagementAdapter>,

    // Settings
    pub settings_storage: Arc<FileSystemSettingsAdapter>,
    pub get_settings: Arc<GetSettingsUseCase<FileSystemSettingsAdapter>>,
    pub save_settings: Arc<SaveSettingsUseCase<FileSystemSettingsAdapter>>,

    // Global Shortcut
    pub global_shortcut: Arc<GlobalShortcutAdapter>,
    pub register_global_shortcut: Arc<RegisterGlobalShortcutUseCase>,
    pub unregister_global_shortcut: Arc<UnregisterGlobalShortcutUseCase>,
    pub check_global_shortcut_availability: Arc<CheckGlobalShortcutAvailabilityUseCase>,

    // Element Scanner
    pub element_scanner: Arc<ElementScannerAdapter>,
    pub scan_elements_use_case: Arc<ScanElementsUseCase>,
    pub click_element_use_case: Arc<ClickElementUseCase>,
    pub focus_element_use_case: Arc<FocusElementUseCase>,
    pub trigger_element_action_use_case: Arc<aumate_application::TriggerElementActionUseCase>,
}
