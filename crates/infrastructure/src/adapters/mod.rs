// 适配器模块

pub mod clipboard;
pub mod element_scanner;
pub mod global_shortcut;
pub mod hotkey;
pub mod image;
pub mod page;
pub mod screen_capture;
pub mod scroll;
pub mod settings;
pub mod storage;
pub mod ui_automation;
pub mod window;
pub mod window_layout;
pub mod window_list;
pub mod window_vibrancy;

// Re-export
pub use clipboard::ClipboardAdapter;
pub use element_scanner::ElementScannerAdapter;
pub use global_shortcut::GlobalShortcutAdapter;
pub use hotkey::{HotkeyListenerAdapter, InputSimulationAdapter};
pub use image::ImageProcessingAdapter;
pub use page::PageManagementAdapter;
pub use screen_capture::ScreenCaptureAdapter;
pub use scroll::ScrollCaptureAdapter;
pub use settings::FileSystemSettingsAdapter;
pub use storage::{FileSystemAdapter, MemoryCacheAdapter};
pub use ui_automation::UIAutomationAdapter;
pub use window::WindowManagementAdapter;
pub use window_layout::WindowLayoutAdapter;
pub use window_list::WindowListAdapter;
pub use window_vibrancy::WindowVibrancyAdapter;
