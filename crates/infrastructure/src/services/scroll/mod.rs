// 滚动截图服务
//
// **完整迁移**: 从 app-scroll-screenshot-service (~1081 行)
//
// 提供滚动截图的核心功能，包括图像捕获、匹配和拼接

pub mod scroll_screenshot_capture_service;
pub mod scroll_screenshot_image_service;
pub mod scroll_screenshot_service;

pub use scroll_screenshot_capture_service::ScrollScreenshotCaptureService;
pub use scroll_screenshot_image_service::ScrollScreenshotImageService;
pub use scroll_screenshot_service::{ScrollDirection, ScrollImageList, ScrollScreenshotService};
