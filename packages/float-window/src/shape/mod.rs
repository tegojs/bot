//! Window shape system

mod mask;

pub use mask::ShapeMask;

/// Window shape types
#[derive(Debug, Clone, Default, PartialEq)]
pub enum WindowShape {
    /// Standard rectangular window
    #[default]
    Rectangle,
    /// Circular window
    Circle,
    /// Custom shape defined by a mask image
    /// The alpha channel determines the window shape
    Custom {
        /// RGBA image data for the mask
        mask: Vec<u8>,
        /// Width of the mask image
        width: u32,
        /// Height of the mask image
        height: u32,
    },
}

impl WindowShape {
    /// Create a custom shape from RGBA image data
    pub fn custom(mask: Vec<u8>, width: u32, height: u32) -> Self {
        Self::Custom {
            mask,
            width,
            height,
        }
    }

    /// Check if a point is inside the shape
    pub fn contains(&self, x: f32, y: f32, width: f32, height: f32) -> bool {
        match self {
            WindowShape::Rectangle => {
                x >= 0.0 && x < width && y >= 0.0 && y < height
            }
            WindowShape::Circle => {
                let cx = width / 2.0;
                let cy = height / 2.0;
                let radius = width.min(height) / 2.0;
                let dx = x - cx;
                let dy = y - cy;
                dx * dx + dy * dy <= radius * radius
            }
            WindowShape::Custom {
                mask,
                width: mw,
                height: mh,
            } => {
                // Scale point to mask coordinates
                let mx = (x / width * (*mw as f32)) as u32;
                let my = (y / height * (*mh as f32)) as u32;
                if mx >= *mw || my >= *mh {
                    return false;
                }
                // Check alpha channel (RGBA, so alpha is at index 3)
                let idx = ((my * mw + mx) * 4 + 3) as usize;
                mask.get(idx).map(|&a| a > 128).unwrap_or(false)
            }
        }
    }
}
