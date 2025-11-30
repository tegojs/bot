//! Event handler

use super::{EventCallback, FloatingWindowEvent};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Event handler for managing window event callbacks
pub struct EventHandler {
    callbacks: Arc<RwLock<HashMap<String, Vec<EventCallback>>>>,
    next_id: Arc<RwLock<usize>>,
}

impl Default for EventHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl EventHandler {
    pub fn new() -> Self {
        Self {
            callbacks: Arc::new(RwLock::new(HashMap::new())),
            next_id: Arc::new(RwLock::new(0)),
        }
    }

    /// Register an event callback
    pub fn on<F>(&self, event_type: &str, callback: F) -> usize
    where
        F: Fn(&FloatingWindowEvent) + Send + Sync + 'static,
    {
        let mut callbacks = self.callbacks.write().unwrap();
        let mut id = self.next_id.write().unwrap();
        let current_id = *id;
        *id += 1;

        callbacks
            .entry(event_type.to_string())
            .or_default()
            .push(Box::new(callback));

        current_id
    }

    /// Dispatch an event to all registered callbacks
    pub fn dispatch(&self, event: &FloatingWindowEvent) {
        let event_type = self.event_type_name(event);
        let callbacks = self.callbacks.read().unwrap();

        // Dispatch to specific event type callbacks
        if let Some(handlers) = callbacks.get(event_type) {
            for handler in handlers {
                handler(event);
            }
        }

        // Also dispatch to "all" listeners
        if let Some(handlers) = callbacks.get("all") {
            for handler in handlers {
                handler(event);
            }
        }
    }

    /// Get the event type name for an event
    fn event_type_name(&self, event: &FloatingWindowEvent) -> &'static str {
        match event {
            FloatingWindowEvent::Show => "show",
            FloatingWindowEvent::Hide => "hide",
            FloatingWindowEvent::Close => "close",
            FloatingWindowEvent::Move { .. } => "move",
            FloatingWindowEvent::Resize { .. } => "resize",
            FloatingWindowEvent::Click { .. } => "click",
            FloatingWindowEvent::DragStart { .. } => "drag-start",
            FloatingWindowEvent::Drag { .. } => "drag",
            FloatingWindowEvent::DragEnd { .. } => "drag-end",
            FloatingWindowEvent::MouseEnter => "mouse-enter",
            FloatingWindowEvent::MouseLeave => "mouse-leave",
            FloatingWindowEvent::MouseMove { .. } => "mouse-move",
        }
    }

    /// Clear all callbacks
    pub fn clear(&self) {
        self.callbacks.write().unwrap().clear();
    }
}

impl Clone for EventHandler {
    fn clone(&self) -> Self {
        Self {
            callbacks: Arc::clone(&self.callbacks),
            next_id: Arc::clone(&self.next_id),
        }
    }
}
