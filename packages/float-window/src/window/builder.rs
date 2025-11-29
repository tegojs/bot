//! FloatingWindow builder pattern

use super::config::{Position, Size, WindowConfig, WindowLevel};
use super::FloatingWindow;
use crate::animation::WindowAnimation;
use crate::content::Content;
use crate::effect::{PresetEffect, PresetEffectOptions};
use crate::event::FloatingWindowEvent;
use crate::icon::WindowIcon;
use crate::shape::WindowShape;

/// Builder for creating FloatingWindow instances
pub struct FloatingWindowBuilder {
    config: WindowConfig,
    event_callback: Option<Box<dyn Fn(&FloatingWindowEvent) + Send + Sync>>,
}

impl Default for FloatingWindowBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl FloatingWindowBuilder {
    /// Create a new builder with default configuration
    pub fn new() -> Self {
        Self {
            config: WindowConfig::new(),
            event_callback: None,
        }
    }

    /// Set the window ID
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.config.id = Some(id.into());
        self
    }

    /// Set the window title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.config.title = Some(title.into());
        self
    }

    /// Set the window position
    pub fn position(mut self, x: f64, y: f64) -> Self {
        self.config.position = Position::new(x, y);
        self
    }

    /// Set the window size
    pub fn size(mut self, width: u32, height: u32) -> Self {
        self.config.size = Size::new(width, height);
        self
    }

    /// Set the window shape
    pub fn shape(mut self, shape: WindowShape) -> Self {
        self.config.shape = shape;
        self
    }

    /// Set whether the window is draggable
    pub fn draggable(mut self, draggable: bool) -> Self {
        self.config.draggable = draggable;
        self
    }

    /// Set whether the window is resizable
    pub fn resizable(mut self, resizable: bool) -> Self {
        self.config.resizable = resizable;
        self
    }

    /// Set whether clicks pass through to windows below
    pub fn click_through(mut self, click_through: bool) -> Self {
        self.config.click_through = click_through;
        self
    }

    /// Set the window level
    pub fn level(mut self, level: WindowLevel) -> Self {
        self.config.level = level;
        self
    }

    /// Set always on top
    pub fn always_on_top(mut self, always: bool) -> Self {
        self.config.level = if always {
            WindowLevel::AlwaysOnTop
        } else {
            WindowLevel::Normal
        };
        self
    }

    /// Set the window opacity
    pub fn opacity(mut self, opacity: f32) -> Self {
        self.config.opacity = opacity.clamp(0.0, 1.0);
        self
    }

    /// Set the window icon
    pub fn icon(mut self, icon: WindowIcon) -> Self {
        self.config.icon = Some(icon);
        self
    }

    /// Set the window content
    pub fn content(mut self, content: Content) -> Self {
        self.config.content = Some(content);
        self
    }

    /// Set a particle effect
    pub fn effect(mut self, effect: PresetEffect, options: PresetEffectOptions) -> Self {
        self.config.effect = Some((effect, options));
        self
    }

    /// Set the show animation
    pub fn show_animation(mut self, animation: WindowAnimation) -> Self {
        self.config.show_animation = Some(animation);
        self
    }

    /// Set the hide animation
    pub fn hide_animation(mut self, animation: WindowAnimation) -> Self {
        self.config.hide_animation = Some(animation);
        self
    }

    /// Add an event callback
    pub fn on_event<F>(mut self, callback: F) -> Self
    where
        F: Fn(&FloatingWindowEvent) + Send + Sync + 'static,
    {
        self.event_callback = Some(Box::new(callback));
        self
    }

    /// Build the FloatingWindow
    pub fn build(self) -> Result<FloatingWindow, String> {
        FloatingWindow::from_config(self.config, self.event_callback)
    }
}
