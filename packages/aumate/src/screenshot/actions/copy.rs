//! Copy action - copies screenshot to clipboard

use crate::screenshot::action::{ActionContext, ActionResult, ScreenAction};

/// Action to copy screenshot to clipboard
pub struct CopyAction;

impl CopyAction {
    pub fn new() -> Self {
        Self
    }
}

impl Default for CopyAction {
    fn default() -> Self {
        Self::new()
    }
}

impl ScreenAction for CopyAction {
    fn id(&self) -> &str {
        "copy"
    }

    fn name(&self) -> &str {
        "Copy"
    }

    fn icon(&self) -> Option<&[u8]> {
        // TODO: Add icon bytes
        None
    }

    fn on_click(&mut self, ctx: &ActionContext) -> ActionResult {
        let region = match ctx.get_selected_region() {
            Some(r) => r,
            None => return ActionResult::Failure("No region selected".to_string()),
        };

        // Convert to PNG bytes for clipboard
        let mut png_bytes = Vec::new();
        {
            use image::ImageEncoder;
            let encoder = image::codecs::png::PngEncoder::new(&mut png_bytes);
            if let Err(e) = encoder.write_image(
                region.as_raw(),
                region.width(),
                region.height(),
                image::ExtendedColorType::Rgba8,
            ) {
                return ActionResult::Failure(format!("Failed to encode image: {}", e));
            }
        }

        // Copy to clipboard using crate's clipboard module
        match crate::clipboard::set_image(&png_bytes) {
            Ok(()) => ActionResult::Exit,
            Err(e) => ActionResult::Failure(format!("Failed to copy to clipboard: {}", e)),
        }
    }
}
