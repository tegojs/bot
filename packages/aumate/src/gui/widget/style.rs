//! Widget styling types

/// Spacing (top, right, bottom, left) - CSS-like box model
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Spacing {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl Spacing {
    /// Create uniform spacing on all sides
    pub fn all(value: f32) -> Self {
        Self { top: value, right: value, bottom: value, left: value }
    }

    /// Create symmetric spacing (vertical, horizontal)
    pub fn symmetric(vertical: f32, horizontal: f32) -> Self {
        Self { top: vertical, right: horizontal, bottom: vertical, left: horizontal }
    }

    /// Create spacing with explicit values
    pub fn new(top: f32, right: f32, bottom: f32, left: f32) -> Self {
        Self { top, right, bottom, left }
    }
}

/// Text alignment options
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TextAlign {
    #[default]
    Left,
    Center,
    Right,
}

/// Widget style configuration
#[derive(Debug, Clone, Default, PartialEq)]
pub struct WidgetStyle {
    /// Outer margin (space outside the widget)
    pub margin: Spacing,
    /// Inner padding (space inside the widget)
    pub padding: Spacing,
    /// Minimum width constraint
    pub min_width: Option<f32>,
    /// Minimum height constraint
    pub min_height: Option<f32>,
    /// Maximum width constraint
    pub max_width: Option<f32>,
    /// Maximum height constraint
    pub max_height: Option<f32>,
    /// Background color (RGBA, 0-255)
    pub background_color: Option<[u8; 4]>,
    /// Text/foreground color (RGBA, 0-255)
    pub text_color: Option<[u8; 4]>,
    /// Font size in points
    pub font_size: Option<f32>,
    /// Font family name (e.g., "Helvetica", "Arial")
    pub font_family: Option<String>,
    /// Text alignment
    pub text_align: Option<TextAlign>,
    /// Border radius for rounded corners
    pub border_radius: Option<f32>,
    /// Border width
    pub border_width: Option<f32>,
    /// Border color (RGBA, 0-255)
    pub border_color: Option<[u8; 4]>,
}

impl WidgetStyle {
    /// Create a new empty style
    pub fn new() -> Self {
        Self::default()
    }

    /// Set margin on all sides
    pub fn margin(mut self, value: f32) -> Self {
        self.margin = Spacing::all(value);
        self
    }

    /// Set margin with explicit values
    pub fn margin_trbl(mut self, top: f32, right: f32, bottom: f32, left: f32) -> Self {
        self.margin = Spacing::new(top, right, bottom, left);
        self
    }

    /// Set padding on all sides
    pub fn padding(mut self, value: f32) -> Self {
        self.padding = Spacing::all(value);
        self
    }

    /// Set padding with explicit values
    pub fn padding_trbl(mut self, top: f32, right: f32, bottom: f32, left: f32) -> Self {
        self.padding = Spacing::new(top, right, bottom, left);
        self
    }

    /// Set minimum width
    pub fn min_width(mut self, width: f32) -> Self {
        self.min_width = Some(width);
        self
    }

    /// Set minimum height
    pub fn min_height(mut self, height: f32) -> Self {
        self.min_height = Some(height);
        self
    }

    /// Set maximum width
    pub fn max_width(mut self, width: f32) -> Self {
        self.max_width = Some(width);
        self
    }

    /// Set maximum height
    pub fn max_height(mut self, height: f32) -> Self {
        self.max_height = Some(height);
        self
    }

    /// Set background color from RGBA values (0-255)
    pub fn background_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.background_color = Some([r, g, b, a]);
        self
    }

    /// Set background color from hex string (e.g., "#FF0000" or "#FF0000FF")
    pub fn background_hex(mut self, hex: &str) -> Self {
        self.background_color = parse_hex_color(hex);
        self
    }

    /// Set text color from RGBA values (0-255)
    pub fn text_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.text_color = Some([r, g, b, a]);
        self
    }

    /// Set text color from hex string
    pub fn text_hex(mut self, hex: &str) -> Self {
        self.text_color = parse_hex_color(hex);
        self
    }

    /// Set font size
    pub fn font_size(mut self, size: f32) -> Self {
        self.font_size = Some(size);
        self
    }

    /// Set font family
    pub fn font_family(mut self, family: impl Into<String>) -> Self {
        self.font_family = Some(family.into());
        self
    }

    /// Set text alignment
    pub fn text_align(mut self, align: TextAlign) -> Self {
        self.text_align = Some(align);
        self
    }

    /// Set border radius
    pub fn border_radius(mut self, radius: f32) -> Self {
        self.border_radius = Some(radius);
        self
    }

    /// Set border width
    pub fn border_width(mut self, width: f32) -> Self {
        self.border_width = Some(width);
        self
    }

    /// Set border color
    pub fn border_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.border_color = Some([r, g, b, a]);
        self
    }
}

/// Parse hex color string to RGBA array
fn parse_hex_color(hex: &str) -> Option<[u8; 4]> {
    let hex = hex.trim_start_matches('#');
    match hex.len() {
        6 => {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            Some([r, g, b, 255])
        }
        8 => {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            let a = u8::from_str_radix(&hex[6..8], 16).ok()?;
            Some([r, g, b, a])
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spacing_all() {
        let s = Spacing::all(10.0);
        assert_eq!(s.top, 10.0);
        assert_eq!(s.right, 10.0);
        assert_eq!(s.bottom, 10.0);
        assert_eq!(s.left, 10.0);
    }

    #[test]
    fn test_parse_hex_color() {
        assert_eq!(parse_hex_color("#FF0000"), Some([255, 0, 0, 255]));
        assert_eq!(parse_hex_color("00FF00"), Some([0, 255, 0, 255]));
        assert_eq!(parse_hex_color("#0000FF80"), Some([0, 0, 255, 128]));
        assert_eq!(parse_hex_color("invalid"), None);
    }

    #[test]
    fn test_widget_style_builder() {
        let style =
            WidgetStyle::new().margin(10.0).padding(5.0).min_width(100.0).background_hex("#FF0000");

        assert_eq!(style.margin.top, 10.0);
        assert_eq!(style.padding.left, 5.0);
        assert_eq!(style.min_width, Some(100.0));
        assert_eq!(style.background_color, Some([255, 0, 0, 255]));
    }
}
