//! Window icon system

mod preset;

use std::path::Path;

pub use preset::IconName;

/// Window icon types
#[derive(Debug, Clone)]
pub enum WindowIcon {
    /// Emoji icon (rendered as text)
    Emoji(String),
    /// Preset icon from built-in set
    Preset(IconName),
    /// Custom image icon
    Image {
        /// RGBA image data
        data: Vec<u8>,
        /// Image width
        width: u32,
        /// Image height
        height: u32,
    },
}

impl WindowIcon {
    /// Create an emoji icon
    pub fn emoji(emoji: impl Into<String>) -> Self {
        Self::Emoji(emoji.into())
    }

    /// Create a preset icon
    pub fn preset(name: IconName) -> Self {
        Self::Preset(name)
    }

    /// Create a custom image icon from RGBA data
    pub fn image(data: Vec<u8>, width: u32, height: u32) -> Self {
        Self::Image { data, width, height }
    }

    /// Load icon from file path (PNG/SVG)
    ///
    /// The image will be resized to 64x64 pixels for use as an icon.
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, String> {
        let loaded = crate::gui::util::load_image(path.as_ref(), Some(64), Some(64))?;
        Ok(Self::Image { data: loaded.data, width: loaded.width, height: loaded.height })
    }

    /// Load icon from file path with custom size
    pub fn from_path_sized(
        path: impl AsRef<Path>,
        width: u32,
        height: u32,
    ) -> Result<Self, String> {
        let loaded = crate::gui::util::load_image(path.as_ref(), Some(width), Some(height))?;
        Ok(Self::Image { data: loaded.data, width: loaded.width, height: loaded.height })
    }
}
