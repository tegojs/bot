// Port 接口定义
// 这些接口在 Domain 层定义，在 Infrastructure 层实现

pub mod clipboard;
pub mod element_scanner;
pub mod global_shortcut;
pub mod hotkey;
pub mod page;
pub mod platform;
pub mod screenshot;
pub mod scroll;
pub mod settings;
pub mod storage;
pub mod window;

// Re-export for convenience
pub use clipboard::ClipboardPort;
pub use element_scanner::{ElementScannerPort, ElementType, ScannableElement};
pub use global_shortcut::GlobalShortcutPort;
pub use hotkey::{HotkeyListenerPort, InputEventHandler, InputSimulationPort};
pub use page::PageManagementPort;
pub use platform::PlatformInfoPort;
pub use screenshot::{ImageProcessingPort, ScreenCapturePort};
pub use scroll::ScrollCapturePort;
pub use settings::SettingsStoragePort;
pub use storage::{CachePort, FileSystemPort};
pub use window::{MonitorInfo, UIAutomationPort, WindowLayout, WindowLayoutPort, WindowListPort, WindowManagementPort};
