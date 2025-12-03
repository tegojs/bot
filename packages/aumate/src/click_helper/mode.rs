//! Click Helper mode state machine
//!
//! Manages the Click Helper state: scanning, input handling, and click execution.

use super::accessibility::{AccessibilityProvider, create_provider};
use super::config::ClickHelperConfig;
use super::overlay::ClickHelperOverlay;
use crate::error::Result;

/// Click Helper state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClickHelperState {
    /// Not active
    Inactive,
    /// Scanning for clickable elements
    Scanning,
    /// Showing overlay, waiting for input
    WaitingForInput,
    /// Processing click
    Clicking,
    /// Exiting mode
    Exiting,
}

/// Action result from Click Helper
#[derive(Debug, Clone)]
pub enum ClickHelperAction {
    /// Continue waiting for input
    Continue,
    /// Click at the specified position
    Click((f32, f32)),
    /// No match found, input reset
    NoMatch,
    /// Mode cancelled by user
    Cancelled,
    /// Error occurred
    Error(String),
}

/// Click Helper mode controller
pub struct ClickHelperMode {
    /// Current state
    state: ClickHelperState,
    /// Configuration
    config: ClickHelperConfig,
    /// Overlay (only exists when active)
    overlay: Option<ClickHelperOverlay>,
    /// Accessibility provider
    accessibility: Box<dyn AccessibilityProvider>,
}

impl ClickHelperMode {
    /// Create a new Click Helper mode
    pub fn new() -> Self {
        Self {
            state: ClickHelperState::Inactive,
            config: ClickHelperConfig::load().unwrap_or_default(),
            overlay: None,
            accessibility: create_provider(),
        }
    }

    /// Create with a specific config
    pub fn with_config(config: ClickHelperConfig) -> Self {
        Self {
            state: ClickHelperState::Inactive,
            config,
            overlay: None,
            accessibility: create_provider(),
        }
    }

    /// Get current state
    pub fn state(&self) -> ClickHelperState {
        self.state
    }

    /// Check if mode is active
    pub fn is_active(&self) -> bool {
        self.state != ClickHelperState::Inactive
    }

    /// Get the config
    pub fn config(&self) -> &ClickHelperConfig {
        &self.config
    }

    /// Update the config
    pub fn set_config(&mut self, config: ClickHelperConfig) {
        self.config = config;
    }

    /// Check if accessibility is trusted
    pub fn is_accessibility_trusted(&self) -> bool {
        self.accessibility.is_trusted()
    }

    /// Request accessibility permission
    pub fn request_accessibility_permission(&self) {
        self.accessibility.request_permission();
    }

    /// Activate Click Helper mode
    pub fn activate(&mut self) -> Result<()> {
        if self.state != ClickHelperState::Inactive {
            return Ok(());
        }

        if !self.accessibility.is_trusted() {
            self.accessibility.request_permission();
        }

        log::info!("Activating Click Helper mode");
        self.state = ClickHelperState::Scanning;

        // Get clickable elements
        match self.accessibility.get_clickable_elements() {
            Ok(elements) => {
                if elements.is_empty() {
                    log::warn!("No clickable elements found");
                    self.state = ClickHelperState::Inactive;
                    return Ok(());
                }

                log::info!("Found {} clickable elements", elements.len());

                // Create overlay with hints
                self.overlay = Some(ClickHelperOverlay::new(elements, &self.config));
                self.state = ClickHelperState::WaitingForInput;

                Ok(())
            }
            Err(e) => {
                log::error!("Error getting clickable elements: {}", e);
                self.state = ClickHelperState::Inactive;
                Err(e)
            }
        }
    }

    /// Deactivate Click Helper mode
    pub fn deactivate(&mut self) {
        log::info!("Deactivating Click Helper mode");
        self.overlay = None;
        self.state = ClickHelperState::Inactive;
    }

    /// Handle a key press
    pub fn handle_key(&mut self, c: char) -> ClickHelperAction {
        if self.state != ClickHelperState::WaitingForInput {
            return ClickHelperAction::Continue;
        }

        let Some(ref mut overlay) = self.overlay else {
            return ClickHelperAction::Continue;
        };

        // Add character to input
        overlay.add_input(c);

        // Check for matches
        let match_count = overlay.matching_count();

        match match_count {
            0 => {
                // No matches - reset input
                log::debug!("No match for input, resetting");
                overlay.reset_input();
                ClickHelperAction::NoMatch
            }
            1 => {
                // Single match - check if it's exact
                if let Some(element) = overlay.get_unique_match() {
                    log::info!("Match found, clicking at {:?}", element.position);
                    let pos = element.position;
                    self.deactivate();
                    ClickHelperAction::Click(pos)
                } else {
                    ClickHelperAction::Continue
                }
            }
            _ => {
                // Multiple matches - wait for more input
                // But check if current input exactly matches one hint
                if let Some(element) = overlay.get_exact_match() {
                    log::info!("Exact match found, clicking at {:?}", element.position);
                    let pos = element.position;
                    self.deactivate();
                    ClickHelperAction::Click(pos)
                } else {
                    ClickHelperAction::Continue
                }
            }
        }
    }

    /// Handle backspace key
    pub fn handle_backspace(&mut self) -> ClickHelperAction {
        if let Some(ref mut overlay) = self.overlay {
            if overlay.input_buffer().is_empty() {
                // If input is already empty, cancel
                self.deactivate();
                return ClickHelperAction::Cancelled;
            }
            overlay.backspace();
        }
        ClickHelperAction::Continue
    }

    /// Handle escape key
    pub fn handle_escape(&mut self) -> ClickHelperAction {
        self.deactivate();
        ClickHelperAction::Cancelled
    }

    /// Render the overlay (if active)
    pub fn render(&self, ctx: &egui::Context) {
        if let Some(ref overlay) = self.overlay {
            overlay.render(ctx);
        }
    }

    /// Get overlay for testing
    #[cfg(test)]
    pub fn overlay(&self) -> Option<&ClickHelperOverlay> {
        self.overlay.as_ref()
    }
}

impl Default for ClickHelperMode {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_state() {
        let mode = ClickHelperMode::new();
        assert_eq!(mode.state(), ClickHelperState::Inactive);
        assert!(!mode.is_active());
    }

    #[test]
    fn test_deactivate() {
        let mut mode = ClickHelperMode::new();
        mode.deactivate();
        assert_eq!(mode.state(), ClickHelperState::Inactive);
    }
}
