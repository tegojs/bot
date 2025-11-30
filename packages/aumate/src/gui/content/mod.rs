//! Content rendering system

mod custom;
mod image;
mod text;

use std::path::Path;

pub use custom::ContentRenderer;
pub use image::{ImageDisplayOptions, ScaleMode};
pub use text::{TextAlign, TextDisplayOptions};

/// Window content types
#[derive(Debug, Clone)]
pub enum Content {
    /// Display an image
    Image {
        /// RGBA image data
        data: Vec<u8>,
        /// Image width
        width: u32,
        /// Image height
        height: u32,
        /// Display options
        options: ImageDisplayOptions,
    },
    /// Display text
    Text {
        /// Text content
        text: String,
        /// Display options
        options: TextDisplayOptions,
    },
    /// Custom renderer (not clonable, use Content::custom())
    Custom,
}

impl Default for Content {
    fn default() -> Self {
        Content::Text { text: String::new(), options: TextDisplayOptions::default() }
    }
}

impl Content {
    /// Create image content from RGBA data
    pub fn image(data: Vec<u8>, width: u32, height: u32) -> Self {
        Self::Image { data, width, height, options: ImageDisplayOptions::default() }
    }

    /// Create image content with options
    pub fn image_with_options(
        data: Vec<u8>,
        width: u32,
        height: u32,
        options: ImageDisplayOptions,
    ) -> Self {
        Self::Image { data, width, height, options }
    }

    /// Create text content
    pub fn text(text: impl Into<String>) -> Self {
        Self::Text { text: text.into(), options: TextDisplayOptions::default() }
    }

    /// Create text content with options
    pub fn text_with_options(text: impl Into<String>, options: TextDisplayOptions) -> Self {
        Self::Text { text: text.into(), options }
    }

    /// Load content from image file (PNG/SVG)
    ///
    /// Loads the image at its original size.
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, String> {
        let loaded = crate::gui::util::load_image(path.as_ref(), None, None)?;
        Ok(Self::Image {
            data: loaded.data,
            width: loaded.width,
            height: loaded.height,
            options: ImageDisplayOptions::default(),
        })
    }

    /// Load content with specific size (stretched)
    ///
    /// Loads and resizes the image to the specified dimensions.
    pub fn from_path_sized(
        path: impl AsRef<Path>,
        width: u32,
        height: u32,
    ) -> Result<Self, String> {
        let loaded = crate::gui::util::load_image(path.as_ref(), Some(width), Some(height))?;
        Ok(Self::Image {
            data: loaded.data,
            width: loaded.width,
            height: loaded.height,
            options: ImageDisplayOptions::new().with_scale_mode(ScaleMode::Stretch),
        })
    }
}
