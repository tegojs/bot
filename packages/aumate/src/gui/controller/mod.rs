//! Controller module for the aumate GUI
//!
//! This module provides a modular controller system with:
//! - Dynamic tab registration via FeatureRegistry
//! - Trait-based features (ControllerFeature)
//! - Async task support for non-blocking operations
//! - Dependency injection via ControllerContext

mod async_task;
mod context;
mod core;
mod feature;
mod floating_windows;
mod registry;
mod settings;
mod types;

// Re-export public API
pub use async_task::AsyncTask;
pub use context::ControllerContext;
pub use core::{
    CONTROLLER_ICON_SIZE, format_time_ago, load_default_background, render_navigation,
    render_title_bar,
};
pub use feature::{ControllerFeature, FeatureState};
pub use registry::FeatureRegistry;
pub use types::{TabId, TabInfo};

use crate::gui::content::Content;
use crate::gui::window::{CommandSender, WindowRegistry};
use egui::TextureHandle;
use std::collections::HashMap;

/// Main controller state that orchestrates all features.
///
/// The controller provides a unified interface for the GUI, managing:
/// - Feature registration and tab navigation
/// - Shared resources (textures, background)
/// - Communication with the window system
pub struct ControllerState {
    /// Command sender for window operations
    command_sender: CommandSender,

    /// Window registry for managing floating windows
    window_registry: WindowRegistry,

    /// Shared texture cache for icons
    icon_cache: HashMap<String, TextureHandle>,

    /// Controller background image
    controller_background: Option<Content>,

    /// Feature registry with dynamic tabs
    features: FeatureRegistry,

    /// Whether the controller has been initialized
    initialized: bool,
}

impl ControllerState {
    /// Create a new controller state with the default feature registry
    pub fn new(command_sender: CommandSender, window_registry: WindowRegistry) -> Self {
        let features = create_default_registry();

        Self {
            command_sender,
            window_registry,
            icon_cache: HashMap::new(),
            controller_background: load_default_background(),
            features,
            initialized: false,
        }
    }

    /// Get a reference to the window registry
    pub fn registry(&self) -> &WindowRegistry {
        &self.window_registry
    }

    /// Get a mutable reference to the window registry
    pub fn registry_mut(&mut self) -> &mut WindowRegistry {
        &mut self.window_registry
    }

    /// Get the controller background
    pub fn background(&self) -> Option<&Content> {
        self.controller_background.as_ref()
    }

    /// Initialize all features (called once on first render)
    fn ensure_initialized(&mut self, egui_ctx: &egui::Context) {
        if self.initialized {
            return;
        }

        let mut ctx = ControllerContext {
            command_sender: &self.command_sender,
            registry: &mut self.window_registry,
            icon_cache: &mut self.icon_cache,
            egui_ctx,
        };

        // Initialize all features
        if let Err(e) = self.features.initialize_all(&mut ctx) {
            log::error!("Failed to initialize features: {}", e);
        }

        // Activate the first tab
        self.features.activate_first(&mut ctx);

        self.initialized = true;
    }

    /// Render the controller UI
    pub fn render(&mut self, ctx: &egui::Context) {
        // Ensure features are initialized
        self.ensure_initialized(ctx);

        // Define frame styles
        let title_frame = egui::Frame::new()
            .fill(egui::Color32::from_rgba_unmultiplied(20, 20, 25, 240))
            .inner_margin(egui::Margin::symmetric(12, 8));

        let nav_frame = egui::Frame::new()
            .fill(egui::Color32::from_rgba_unmultiplied(25, 25, 30, 230))
            .inner_margin(egui::Margin::symmetric(8, 4));

        let content_frame = egui::Frame::new()
            .fill(egui::Color32::from_rgba_unmultiplied(30, 30, 35, 220))
            .inner_margin(egui::Margin::same(16));

        // Update all features (for async callbacks)
        {
            let mut ctx_for_update = ControllerContext {
                command_sender: &self.command_sender,
                registry: &mut self.window_registry,
                icon_cache: &mut self.icon_cache,
                egui_ctx: ctx,
            };
            self.features.update_all(&mut ctx_for_update);
        }

        // TOP PANEL: Title bar
        egui::TopBottomPanel::top("title_bar").frame(title_frame).show(ctx, |ui| {
            let controller_ctx = ControllerContext {
                command_sender: &self.command_sender,
                registry: &mut self.window_registry,
                icon_cache: &mut self.icon_cache,
                egui_ctx: ctx,
            };
            render_title_bar(ui, &controller_ctx);
        });

        // LEFT PANEL: Navigation
        egui::SidePanel::left("navigation")
            .resizable(false)
            .exact_width(160.0)
            .frame(nav_frame)
            .show(ctx, |ui| {
                let mut controller_ctx = ControllerContext {
                    command_sender: &self.command_sender,
                    registry: &mut self.window_registry,
                    icon_cache: &mut self.icon_cache,
                    egui_ctx: ctx,
                };
                render_navigation(ui, &mut self.features, &mut controller_ctx);
            });

        // CENTRAL PANEL: Content based on active tab
        egui::CentralPanel::default().frame(content_frame).show(ctx, |ui| {
            egui::ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {
                let mut controller_ctx = ControllerContext {
                    command_sender: &self.command_sender,
                    registry: &mut self.window_registry,
                    icon_cache: &mut self.icon_cache,
                    egui_ctx: ctx,
                };
                self.features.render_active(ui, &mut controller_ctx);
            });
        });
    }

    /// Shutdown the controller and all features
    pub fn shutdown(&mut self) {
        self.features.shutdown_all();
    }
}

/// Create the default feature registry with all available features.
///
/// Features are registered based on compile-time feature flags.
pub fn create_default_registry() -> FeatureRegistry {
    let mut registry = FeatureRegistry::new();

    // Core features (always present)
    registry.register(Box::new(floating_windows::FloatingWindowsFeature::new()));
    registry.register(Box::new(settings::SettingsFeature::new()));

    // Screenshot feature
    registry.register(Box::new(crate::screenshot::ScreenshotFeature::new()));

    // Menu bar feature
    registry.register(Box::new(crate::gui::menu_bar::MenuBarFeature::new()));

    // Clipboard feature
    registry.register(Box::new(crate::clipboard_manager::ClipboardFeature::new()));

    // STT feature (feature-gated)
    #[cfg(feature = "stt")]
    registry.register(Box::new(crate::stt::SttFeature::new()));

    // OCR feature (feature-gated)
    #[cfg(feature = "ocr")]
    registry.register(Box::new(crate::ocr::OcrFeature::new()));

    // Click Helper feature (feature-gated + macOS only)
    #[cfg(all(feature = "click_helper", target_os = "macos"))]
    registry.register(Box::new(crate::click_helper::ClickHelperFeature::new()));

    registry
}
