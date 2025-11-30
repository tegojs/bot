//! Custom content renderer

use egui::Ui;

/// Trait for custom content rendering
pub trait ContentRenderer: Send + Sync {
    /// Render the content using egui
    fn render(&self, ui: &mut Ui);
}

/// A boxed content renderer
#[allow(dead_code)]
pub type BoxedRenderer = Box<dyn ContentRenderer>;
