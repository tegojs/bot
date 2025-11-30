//! Image content display

/// Scale mode for images
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ScaleMode {
    /// Scale to fit within bounds, maintaining aspect ratio
    #[default]
    Fit,
    /// Scale to fill bounds, maintaining aspect ratio (may crop)
    Fill,
    /// Stretch to fill bounds (ignores aspect ratio)
    Stretch,
    /// Center without scaling
    Center,
}

/// Image display options
#[derive(Debug, Clone, Default)]
pub struct ImageDisplayOptions {
    /// How to scale the image
    pub scale_mode: ScaleMode,
    /// Background color (RGBA)
    pub background_color: Option<[u8; 4]>,
    /// Whether to maintain aspect ratio
    pub maintain_aspect_ratio: bool,
}

impl ImageDisplayOptions {
    pub fn new() -> Self {
        Self { scale_mode: ScaleMode::Fit, background_color: None, maintain_aspect_ratio: true }
    }

    pub fn with_scale_mode(mut self, mode: ScaleMode) -> Self {
        self.scale_mode = mode;
        self
    }

    pub fn with_background(mut self, color: [u8; 4]) -> Self {
        self.background_color = Some(color);
        self
    }
}
