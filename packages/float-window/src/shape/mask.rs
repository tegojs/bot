//! Shape mask for hit testing

use super::WindowShape;

/// Shape mask for efficient hit testing
pub struct ShapeMask {
    shape: WindowShape,
    width: f32,
    height: f32,
}

impl ShapeMask {
    pub fn new(shape: WindowShape, width: f32, height: f32) -> Self {
        Self {
            shape,
            width,
            height,
        }
    }

    /// Check if a point is inside the shape
    pub fn contains(&self, x: f32, y: f32) -> bool {
        self.shape.contains(x, y, self.width, self.height)
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
