//! Mouse control module
//!
//! Provides mouse movement, clicking, dragging, and scrolling functionality.

use crate::error::{AumateError, Result};
use enigo::{Axis, Button, Coordinate, Direction, Enigo, Mouse as MouseTrait};
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/// Mouse button types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

impl FromStr for MouseButton {
    type Err = AumateError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "left" => Ok(MouseButton::Left),
            "right" => Ok(MouseButton::Right),
            "middle" => Ok(MouseButton::Middle),
            _ => Err(AumateError::Input(format!("Invalid button: {}", s))),
        }
    }
}

impl MouseButton {
    fn to_enigo_button(self) -> Button {
        match self {
            MouseButton::Left => Button::Left,
            MouseButton::Right => Button::Right,
            MouseButton::Middle => Button::Middle,
        }
    }
}

/// Mouse position
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MousePosition {
    pub x: i32,
    pub y: i32,
}

/// Mouse controller
pub struct Mouse {
    enigo: Arc<Mutex<Enigo>>,
    delay_ms: Arc<Mutex<u32>>,
}

impl Mouse {
    /// Create a new mouse controller
    pub fn new() -> Result<Self> {
        let enigo = Enigo::new(&enigo::Settings::default())
            .map_err(|e| AumateError::Input(format!("Failed to create Enigo: {}", e)))?;
        Ok(Self { enigo: Arc::new(Mutex::new(enigo)), delay_ms: Arc::new(Mutex::new(10)) })
    }

    /// Move the mouse to the specified coordinates
    pub fn move_mouse(&self, x: i32, y: i32) -> Result<()> {
        let mut enigo =
            self.enigo.lock().map_err(|e| AumateError::Input(format!("Lock error: {}", e)))?;
        let _ = enigo.move_mouse(x, y, Coordinate::Abs);
        self.apply_delay();
        Ok(())
    }

    /// Move the mouse smoothly to the specified coordinates
    pub fn move_mouse_smooth(&self, x: i32, y: i32) -> Result<()> {
        self.move_mouse_smooth_with_speed(x, y, 3.0)
    }

    /// Move the mouse smoothly with custom speed
    pub fn move_mouse_smooth_with_speed(&self, x: i32, y: i32, speed: f64) -> Result<()> {
        let current_pos = self.get_mouse_pos()?;
        let start_x = current_pos.x as f64;
        let start_y = current_pos.y as f64;
        let end_x = x as f64;
        let end_y = y as f64;

        let distance = ((end_x - start_x).powi(2) + (end_y - start_y).powi(2)).sqrt();
        let steps = (distance / speed).max(1.0) as u32;

        for i in 0..=steps {
            let t = i as f64 / steps as f64;
            // Easing function for smooth movement
            let eased = t * t * (3.0 - 2.0 * t);
            let current_x = (start_x + (end_x - start_x) * eased) as i32;
            let current_y = (start_y + (end_y - start_y) * eased) as i32;

            self.move_mouse(current_x, current_y)?;
            thread::sleep(Duration::from_millis(1));
        }

        Ok(())
    }

    /// Get the current mouse position
    pub fn get_mouse_pos(&self) -> Result<MousePosition> {
        let enigo =
            self.enigo.lock().map_err(|e| AumateError::Input(format!("Lock error: {}", e)))?;
        let (x, y) = enigo
            .location()
            .map_err(|e| AumateError::Input(format!("Failed to get mouse position: {}", e)))?;
        Ok(MousePosition { x, y })
    }

    /// Click the mouse button
    pub fn click(&self, button: MouseButton) -> Result<()> {
        let mut enigo =
            self.enigo.lock().map_err(|e| AumateError::Input(format!("Lock error: {}", e)))?;
        let _ = enigo.button(button.to_enigo_button(), Direction::Click);
        self.apply_delay();
        Ok(())
    }

    /// Double click the mouse button
    pub fn double_click(&self, button: MouseButton) -> Result<()> {
        let mut enigo =
            self.enigo.lock().map_err(|e| AumateError::Input(format!("Lock error: {}", e)))?;
        let enigo_button = button.to_enigo_button();
        let _ = enigo.button(enigo_button, Direction::Click);
        thread::sleep(Duration::from_millis(50));
        let _ = enigo.button(enigo_button, Direction::Click);
        self.apply_delay();
        Ok(())
    }

    /// Click with optional button and double-click support (robotjs compatible)
    pub fn mouse_click(&self, button: Option<&str>, double: Option<bool>) -> Result<()> {
        let button_str = button.unwrap_or("left");
        let is_double = double.unwrap_or(false);
        let mouse_button = MouseButton::from_str(button_str)?;

        if is_double { self.double_click(mouse_button) } else { self.click(mouse_button) }
    }

    /// Press the mouse button down
    pub fn press(&self, button: MouseButton) -> Result<()> {
        let mut enigo =
            self.enigo.lock().map_err(|e| AumateError::Input(format!("Lock error: {}", e)))?;
        let _ = enigo.button(button.to_enigo_button(), Direction::Press);
        self.apply_delay();
        Ok(())
    }

    /// Release the mouse button
    pub fn release(&self, button: MouseButton) -> Result<()> {
        let mut enigo =
            self.enigo.lock().map_err(|e| AumateError::Input(format!("Lock error: {}", e)))?;
        let _ = enigo.button(button.to_enigo_button(), Direction::Release);
        self.apply_delay();
        Ok(())
    }

    /// Toggle mouse button (press or release) - robotjs compatible
    pub fn mouse_toggle(&self, down: &str, button: Option<&str>) -> Result<()> {
        let button_str = button.unwrap_or("left");
        let mouse_button = MouseButton::from_str(button_str)?;

        match down {
            "down" => self.press(mouse_button),
            "up" => self.release(mouse_button),
            _ => Err(AumateError::Input(format!("Invalid direction: {}", down))),
        }
    }

    /// Drag the mouse to the specified coordinates
    pub fn drag_mouse(&self, x: i32, y: i32) -> Result<()> {
        let mut enigo =
            self.enigo.lock().map_err(|e| AumateError::Input(format!("Lock error: {}", e)))?;
        // Press left button
        let _ = enigo.button(Button::Left, Direction::Press);
        // Move to position
        let _ = enigo.move_mouse(x, y, Coordinate::Abs);
        // Release left button
        let _ = enigo.button(Button::Left, Direction::Release);
        self.apply_delay();
        Ok(())
    }

    /// Scroll the mouse wheel
    pub fn scroll(&self, x: i32, y: i32) -> Result<()> {
        let mut enigo =
            self.enigo.lock().map_err(|e| AumateError::Input(format!("Lock error: {}", e)))?;
        if x != 0 {
            let _ = enigo.scroll(x, Axis::Horizontal);
        }
        if y != 0 {
            let _ = enigo.scroll(y, Axis::Vertical);
        }
        self.apply_delay();
        Ok(())
    }

    /// Scroll the mouse wheel (robotjs compatible alias)
    pub fn scroll_mouse(&self, x: i32, y: i32) -> Result<()> {
        self.scroll(x, y)
    }

    /// Set the mouse delay in milliseconds
    pub fn set_delay(&self, delay_ms: u32) -> Result<()> {
        let mut delay =
            self.delay_ms.lock().map_err(|e| AumateError::Input(format!("Lock error: {}", e)))?;
        *delay = delay_ms;
        Ok(())
    }

    /// Set the mouse delay in milliseconds (robotjs compatible alias)
    pub fn set_mouse_delay(&self, delay_ms: u32) -> Result<()> {
        self.set_delay(delay_ms)
    }

    fn apply_delay(&self) {
        let delay = self.delay_ms.lock().ok().map(|d| *d).unwrap_or(10);
        if delay > 0 {
            thread::sleep(Duration::from_millis(delay as u64));
        }
    }
}

impl Default for Mouse {
    fn default() -> Self {
        Self::new().expect("Failed to create Mouse")
    }
}
