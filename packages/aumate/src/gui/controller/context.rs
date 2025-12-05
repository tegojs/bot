//! Controller context for dependency injection

use crate::gui::window::{CommandSender, WindowRegistry};
use egui::TextureHandle;
use std::collections::HashMap;

/// Context passed to controller features for accessing shared resources
pub struct ControllerContext<'a> {
    /// Command sender for window operations
    pub command_sender: &'a CommandSender,

    /// Window registry for managing floating windows
    pub registry: &'a mut WindowRegistry,

    /// Shared texture cache for icons
    pub icon_cache: &'a mut HashMap<String, TextureHandle>,

    /// egui context for repaint requests and other UI operations
    pub egui_ctx: &'a egui::Context,
}

impl<'a> ControllerContext<'a> {
    /// Request a UI repaint
    pub fn request_repaint(&self) {
        self.egui_ctx.request_repaint();
    }

    /// Get or create a cached texture
    pub fn get_or_create_texture<F>(&mut self, key: &str, create_fn: F) -> Option<&TextureHandle>
    where
        F: FnOnce(&egui::Context) -> Option<TextureHandle>,
    {
        if !self.icon_cache.contains_key(key) {
            if let Some(texture) = create_fn(self.egui_ctx) {
                self.icon_cache.insert(key.to_string(), texture);
            }
        }
        self.icon_cache.get(key)
    }
}
