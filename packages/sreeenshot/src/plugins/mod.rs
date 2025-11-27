pub mod registry;
pub mod save;
pub mod copy;
pub mod cancel;
pub mod annotate;
pub mod text;

pub use registry::PluginRegistry;
pub use save::SavePlugin;
pub use copy::CopyPlugin;
pub use cancel::CancelPlugin;
pub use annotate::AnnotatePlugin;
pub use text::TextPlugin;

use image::ImageBuffer;
use image::Rgba;

pub struct PluginContext {
    pub selection_coords: Option<((u32, u32), (u32, u32))>,
    #[allow(dead_code)]
    pub screenshot: Option<ImageBuffer<Rgba<u8>, Vec<u8>>>,
    pub monitor: Option<xcap::Monitor>,
}

#[derive(Debug, Clone)]
pub enum PluginResult {
    #[allow(dead_code)]
    Success,
    Failure(String),
    Exit,
    Continue,
}

pub trait Plugin: Send + Sync {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn icon(&self) -> Option<&[u8]>;
    fn on_click(&mut self, context: &PluginContext) -> PluginResult;
}

#[derive(Clone)]
pub struct PluginInfo {
    pub id: String,
    #[allow(dead_code)]
    pub name: String,
    pub icon: Option<Vec<u8>>,
}

