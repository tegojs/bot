//! Selection state for screenshot region selection
//!
//! Tracks mouse drag to define a rectangular selection area.

use super::mode::HandlePosition;

/// Selection state for tracking mouse drag
#[derive(Debug, Clone, Default)]
pub struct Selection {
    /// Start position of the drag (logical coordinates)
    start: Option<(f32, f32)>,
    /// Current/end position of the drag (logical coordinates)
    end: Option<(f32, f32)>,
    /// Whether the selection is complete (mouse released)
    completed: bool,
}

impl Selection {
    /// Create a new empty selection
    pub fn new() -> Self {
        Self::default()
    }

    /// Start a new selection at the given position
    pub fn start(&mut self, pos: (f32, f32)) {
        self.start = Some(pos);
        self.end = Some(pos);
        self.completed = false;
    }

    /// Update the selection end position (during drag)
    pub fn update(&mut self, pos: (f32, f32)) {
        if self.start.is_some() && !self.completed {
            self.end = Some(pos);
        }
    }

    /// Finish the selection (mouse released)
    pub fn finish(&mut self) {
        if self.start.is_some() && self.end.is_some() {
            self.completed = true;
        }
    }

    /// Reset the selection to empty state
    pub fn reset(&mut self) {
        self.start = None;
        self.end = None;
        self.completed = false;
    }

    /// Check if selection is in progress (started but not completed)
    pub fn is_in_progress(&self) -> bool {
        self.start.is_some() && !self.completed
    }

    /// Check if selection is complete
    pub fn is_completed(&self) -> bool {
        self.completed
    }

    /// Check if there is any selection (started or completed)
    pub fn has_selection(&self) -> bool {
        self.start.is_some() && self.end.is_some()
    }

    /// Get the normalized bounds in logical coordinates
    ///
    /// Returns ((min_x, min_y), (max_x, max_y)) with min <= max
    pub fn bounds(&self) -> Option<((f32, f32), (f32, f32))> {
        let start = self.start?;
        let end = self.end?;

        let min_x = start.0.min(end.0);
        let min_y = start.1.min(end.1);
        let max_x = start.0.max(end.0);
        let max_y = start.1.max(end.1);

        Some(((min_x, min_y), (max_x, max_y)))
    }

    /// Get the bounds in physical pixel coordinates
    ///
    /// Applies scale factor for DPI conversion
    pub fn bounds_physical(&self, scale: f32) -> Option<((u32, u32), (u32, u32))> {
        let ((min_x, min_y), (max_x, max_y)) = self.bounds()?;

        Some((
            ((min_x * scale) as u32, (min_y * scale) as u32),
            ((max_x * scale) as u32, (max_y * scale) as u32),
        ))
    }

    /// Get the width and height in logical coordinates
    pub fn size(&self) -> Option<(f32, f32)> {
        let ((min_x, min_y), (max_x, max_y)) = self.bounds()?;
        Some((max_x - min_x, max_y - min_y))
    }

    /// Get the width and height in physical pixels
    pub fn size_physical(&self, scale: f32) -> Option<(u32, u32)> {
        let (w, h) = self.size()?;
        Some(((w * scale) as u32, (h * scale) as u32))
    }

    /// Resize selection by moving a handle to new position
    pub fn resize(&mut self, handle: HandlePosition, new_pos: (f32, f32)) {
        let Some(((mut min_x, mut min_y), (mut max_x, mut max_y))) = self.bounds() else {
            return;
        };

        let (nx, ny) = new_pos;

        match handle {
            HandlePosition::TopLeft => {
                min_x = nx;
                min_y = ny;
            }
            HandlePosition::TopCenter => {
                min_y = ny;
            }
            HandlePosition::TopRight => {
                max_x = nx;
                min_y = ny;
            }
            HandlePosition::MiddleLeft => {
                min_x = nx;
            }
            HandlePosition::MiddleRight => {
                max_x = nx;
            }
            HandlePosition::BottomLeft => {
                min_x = nx;
                max_y = ny;
            }
            HandlePosition::BottomCenter => {
                max_y = ny;
            }
            HandlePosition::BottomRight => {
                max_x = nx;
                max_y = ny;
            }
        }

        // Ensure minimum size (10 pixels)
        const MIN_SIZE: f32 = 10.0;
        if max_x - min_x < MIN_SIZE {
            match handle {
                HandlePosition::TopLeft
                | HandlePosition::MiddleLeft
                | HandlePosition::BottomLeft => {
                    min_x = max_x - MIN_SIZE;
                }
                _ => {
                    max_x = min_x + MIN_SIZE;
                }
            }
        }
        if max_y - min_y < MIN_SIZE {
            match handle {
                HandlePosition::TopLeft | HandlePosition::TopCenter | HandlePosition::TopRight => {
                    min_y = max_y - MIN_SIZE;
                }
                _ => {
                    max_y = min_y + MIN_SIZE;
                }
            }
        }

        // Update internal state (store as start/end, bounds() will normalize)
        self.start = Some((min_x, min_y));
        self.end = Some((max_x, max_y));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_lifecycle() {
        let mut sel = Selection::new();
        assert!(!sel.has_selection());
        assert!(!sel.is_in_progress());
        assert!(!sel.is_completed());

        sel.start((10.0, 20.0));
        assert!(sel.has_selection());
        assert!(sel.is_in_progress());
        assert!(!sel.is_completed());

        sel.update((100.0, 200.0));
        assert!(sel.is_in_progress());

        sel.finish();
        assert!(sel.is_completed());
        assert!(!sel.is_in_progress());

        let bounds = sel.bounds().unwrap();
        assert_eq!(bounds, ((10.0, 20.0), (100.0, 200.0)));
    }

    #[test]
    fn test_bounds_normalization() {
        let mut sel = Selection::new();
        // Start from bottom-right, drag to top-left
        sel.start((100.0, 200.0));
        sel.update((10.0, 20.0));
        sel.finish();

        let bounds = sel.bounds().unwrap();
        // Should be normalized
        assert_eq!(bounds, ((10.0, 20.0), (100.0, 200.0)));
    }

    #[test]
    fn test_physical_coords() {
        let mut sel = Selection::new();
        sel.start((10.0, 20.0));
        sel.update((100.0, 200.0));
        sel.finish();

        let physical = sel.bounds_physical(2.0).unwrap();
        assert_eq!(physical, ((20, 40), (200, 400)));
    }
}
