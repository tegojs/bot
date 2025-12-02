//! Save action - saves screenshot to file

use crate::screenshot::action::{ActionContext, ActionResult, ScreenAction};
use image::ImageEncoder;
use rfd::FileDialog;
use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;

/// Action to save screenshot to a file
pub struct SaveAction {
    /// Last used directory for saving
    last_dir: Option<PathBuf>,
}

impl SaveAction {
    pub fn new() -> Self {
        Self { last_dir: None }
    }

    fn save_image(
        &self,
        image: &image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
        path: &PathBuf,
    ) -> Result<(), String> {
        let file = File::create(path).map_err(|e| format!("Failed to create file: {}", e))?;
        let writer = BufWriter::new(file);

        let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("png").to_lowercase();

        match extension.as_str() {
            "jpg" | "jpeg" => {
                let encoder = image::codecs::jpeg::JpegEncoder::new(writer);
                encoder
                    .write_image(
                        image.as_raw(),
                        image.width(),
                        image.height(),
                        image::ExtendedColorType::Rgba8,
                    )
                    .map_err(|e| format!("Failed to encode JPEG: {}", e))?;
            }
            _ => {
                let encoder = image::codecs::png::PngEncoder::new(writer);
                encoder
                    .write_image(
                        image.as_raw(),
                        image.width(),
                        image.height(),
                        image::ExtendedColorType::Rgba8,
                    )
                    .map_err(|e| format!("Failed to encode PNG: {}", e))?;
            }
        }

        Ok(())
    }
}

impl Default for SaveAction {
    fn default() -> Self {
        Self::new()
    }
}

impl ScreenAction for SaveAction {
    fn id(&self) -> &str {
        "save"
    }

    fn name(&self) -> &str {
        "Save"
    }

    fn icon_id(&self) -> Option<&str> {
        Some("save")
    }

    fn on_click(&mut self, ctx: &ActionContext) -> ActionResult {
        // Use get_composited_region to include annotations in the saved image
        let region = match ctx.get_composited_region() {
            Some(r) => r,
            None => return ActionResult::Failure("No region selected".to_string()),
        };

        // Show file dialog
        let mut dialog = FileDialog::new()
            .set_title("Save Screenshot")
            .add_filter("PNG Image", &["png"])
            .add_filter("JPEG Image", &["jpg", "jpeg"])
            .set_file_name("screenshot.png");

        if let Some(ref dir) = self.last_dir {
            dialog = dialog.set_directory(dir);
        }

        match dialog.save_file() {
            Some(path) => {
                // Remember directory for next time
                if let Some(parent) = path.parent() {
                    self.last_dir = Some(parent.to_path_buf());
                }

                match self.save_image(&region, &path) {
                    Ok(()) => ActionResult::Exit,
                    Err(e) => ActionResult::Failure(e),
                }
            }
            None => {
                // User cancelled
                ActionResult::Continue
            }
        }
    }
}
