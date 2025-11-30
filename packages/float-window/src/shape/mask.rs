//! Shape mask for hit testing

use super::WindowShape;

/// Shape mask for efficient hit testing
pub struct ShapeMask {
    shape: WindowShape,
    width: f32,
    height: f32,
    /// Offset from window origin to shape origin
    offset: f32,
}

impl ShapeMask {
    pub fn new(shape: WindowShape, width: f32, height: f32) -> Self {
        Self {
            shape,
            width,
            height,
            offset: 0.0,
        }
    }

    /// Create a shape mask with an offset (for effect margin)
    pub fn new_with_offset(shape: WindowShape, width: f32, height: f32, offset: f32) -> Self {
        Self {
            shape,
            width,
            height,
            offset,
        }
    }

    /// Check if a point is inside the shape (accounting for offset)
    pub fn contains(&self, x: f32, y: f32) -> bool {
        // Adjust coordinates by offset to get position relative to shape origin
        let local_x = x - self.offset;
        let local_y = y - self.offset;
        self.shape.contains(local_x, local_y, self.width, self.height)
    }

    /// Update dimensions
    pub fn set_size(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
    }

    /// Update shape
    pub fn set_shape(&mut self, shape: WindowShape) {
        self.shape = shape;
    }
}
