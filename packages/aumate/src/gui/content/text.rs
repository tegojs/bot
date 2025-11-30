//! Text content display

/// Text alignment
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TextAlign {
    #[default]
    Left,
    Center,
    Right,
}

/// Text display options
#[derive(Debug, Clone)]
pub struct TextDisplayOptions {
    /// Font size in pixels
    pub font_size: f32,
    /// Text color (RGBA)
    pub color: [u8; 4],
    /// Background color (RGBA)
    pub background_color: Option<[u8; 4]>,
    /// Text alignment
    pub align: TextAlign,
    /// Whether to wrap text
    pub wrap: bool,
}

impl Default for TextDisplayOptions {
    fn default() -> Self {
        Self {
            font_size: 14.0,
            color: [255, 255, 255, 255],
            background_color: None,
            align: TextAlign::Left,
            wrap: true,
        }
    }
}

impl TextDisplayOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    pub fn with_color(mut self, color: [u8; 4]) -> Self {
        self.color = color;
        self
    }

    pub fn with_background(mut self, color: [u8; 4]) -> Self {
        self.background_color = Some(color);
        self
    }

    pub fn with_align(mut self, align: TextAlign) -> Self {
        self.align = align;
        self
    }

    pub fn with_wrap(mut self, wrap: bool) -> Self {
        self.wrap = wrap;
        self
    }
}
