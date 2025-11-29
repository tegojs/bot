//! Window manager for handling multiple windows

use super::FloatingWindow;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Manager for multiple floating windows
pub struct FloatingWindowManager {
    windows: Arc<RwLock<HashMap<String, FloatingWindow>>>,
    next_id: Arc<RwLock<usize>>,
}

impl Default for FloatingWindowManager {
    fn default() -> Self {
        Self::new()
    }
}

impl FloatingWindowManager {
    /// Create a new window manager
    pub fn new() -> Self {
        Self {
            windows: Arc::new(RwLock::new(HashMap::new())),
            next_id: Arc::new(RwLock::new(0)),
        }
    }

    /// Generate a unique window ID
    fn generate_id(&self) -> String {
        let mut id = self.next_id.write().unwrap();
        let current = *id;
        *id += 1;
        format!("window_{}", current)
    }

    /// Add a window to the manager
    pub fn add_window(&self, window: FloatingWindow) -> String {
        let id = window
            .id()
            .map(|s| s.to_string())
            .unwrap_or_else(|| self.generate_id());

        self.windows.write().unwrap().insert(id.clone(), window);
        id
    }

    /// Get a window by ID
    pub fn get_window(&self, id: &str) -> Option<FloatingWindow> {
        self.windows.write().unwrap().remove(id)
    }

    /// Remove a window by ID
    pub fn remove_window(&self, id: &str) -> Option<FloatingWindow> {
        self.windows.write().unwrap().remove(id)
    }

    /// Get all window IDs
    pub fn window_ids(&self) -> Vec<String> {
        self.windows.read().unwrap().keys().cloned().collect()
    }

    /// Get the number of windows
    pub fn window_count(&self) -> usize {
        self.windows.read().unwrap().len()
    }

    /// Clear all windows
    pub fn clear(&self) {
        self.windows.write().unwrap().clear();
    }
}

impl Clone for FloatingWindowManager {
    fn clone(&self) -> Self {
        Self {
            windows: Arc::clone(&self.windows),
            next_id: Arc::clone(&self.next_id),
        }
    }
}
