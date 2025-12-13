// 底层服务模块
//
// 这些服务被适配器使用，封装了实际的实现逻辑

pub mod device_events;
pub mod input_simulation;
pub mod key_listener;
pub mod mouse_listener;
pub mod page;
pub mod scroll;

pub use device_events::DeviceEventHandlerService;
pub use input_simulation::EnigoManager;
pub use key_listener::ListenKeyService;
pub use mouse_listener::ListenMouseService;
pub use page::{HotLoadPage, HotLoadPageRoutePushEvent, HotLoadPageService};
pub use scroll::{
    ScrollDirection, ScrollImageList, ScrollScreenshotCaptureService, ScrollScreenshotImageService,
    ScrollScreenshotService,
};
