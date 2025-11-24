// Annotation framework and tools

use super::types::*;

/// Annotation types
#[derive(Debug, Clone)]
pub enum AnnotationType {
    Arrow { start: Position, end: Position, color: RgbColor, width: u32 },
    Rectangle { region: ScreenRegion, color: RgbColor, width: u32, filled: bool },
    Brush { points: Vec<Position>, color: RgbColor, width: u32 },
}

/// Annotation layer containing multiple annotations
#[derive(Debug, Clone)]
pub struct AnnotationLayer {
    pub id: String,
    pub annotation_type: AnnotationType,
    pub timestamp: i64,
}

impl AnnotationLayer {
    pub fn new(annotation_type: AnnotationType) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            annotation_type,
            timestamp: chrono::Utc::now().timestamp(),
        }
    }
}

/// Annotation manager for handling annotation state
#[derive(Debug, Clone)]
pub struct AnnotationManager {
    layers: Vec<AnnotationLayer>,
    undo_stack: Vec<Vec<AnnotationLayer>>,
    redo_stack: Vec<Vec<AnnotationLayer>>,
}

impl AnnotationManager {
    pub fn new() -> Self {
        Self { layers: Vec::new(), undo_stack: Vec::new(), redo_stack: Vec::new() }
    }

    /// Add a new annotation layer
    pub fn add_layer(&mut self, annotation: AnnotationType) {
        // Save current state for undo
        self.undo_stack.push(self.layers.clone());
        self.redo_stack.clear();

        let layer = AnnotationLayer::new(annotation);
        self.layers.push(layer);
    }

    /// Undo last annotation
    pub fn undo(&mut self) -> bool {
        if let Some(previous_state) = self.undo_stack.pop() {
            self.redo_stack.push(self.layers.clone());
            self.layers = previous_state;
            true
        } else {
            false
        }
    }

    /// Redo last undone annotation
    pub fn redo(&mut self) -> bool {
        if let Some(next_state) = self.redo_stack.pop() {
            self.undo_stack.push(self.layers.clone());
            self.layers = next_state;
            true
        } else {
            false
        }
    }

    /// Get all annotation layers
    pub fn get_layers(&self) -> &[AnnotationLayer] {
        &self.layers
    }

    /// Clear all annotations
    pub fn clear(&mut self) {
        self.undo_stack.push(self.layers.clone());
        self.redo_stack.clear();
        self.layers.clear();
    }

    /// Render annotations onto an image buffer
    pub fn render_annotations(
        &self,
        _buffer: &mut [u8],
        _width: u32,
        _height: u32,
    ) -> Result<(), String> {
        // This will use imageproc to draw annotations
        // For now, this is a placeholder
        Ok(())
    }
}

impl Default for AnnotationManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_annotation_manager() {
        let mut manager = AnnotationManager::new();

        // Add annotation
        manager.add_layer(AnnotationType::Rectangle {
            region: ScreenRegion::new(0, 0, 100, 100),
            color: RgbColor { r: 255, g: 0, b: 0 },
            width: 2,
            filled: false,
        });

        assert_eq!(manager.get_layers().len(), 1);

        // Undo
        assert!(manager.undo());
        assert_eq!(manager.get_layers().len(), 0);

        // Redo
        assert!(manager.redo());
        assert_eq!(manager.get_layers().len(), 1);
    }
}
