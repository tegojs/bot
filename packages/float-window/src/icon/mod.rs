//! Window icon system

mod preset;

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
}
