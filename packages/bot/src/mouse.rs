use enigo::{Enigo, Mouse as MouseTrait, Coordinate, Button, Direction, Axis};
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread;

#[napi]
pub struct Mouse {
    enigo: Arc<Mutex<Enigo>>,
    delay_ms: Arc<Mutex<u32>>,
}

#[napi]
impl Mouse {
    #[napi(constructor)]
    pub fn new() -> Result<Self> {
        let enigo = Enigo::new(&enigo::Settings::default())
            .map_err(|e| Error::from_reason(format!("Failed to create Enigo: {}", e)))?;
        Ok(Self {
            enigo: Arc::new(Mutex::new(enigo)),
            delay_ms: Arc::new(Mutex::new(10)),
        })
    }

    /// Move the mouse to the specified coordinates
    #[napi]
    pub fn move_mouse(&self, x: i32, y: i32) -> Result<()> {
        let mut enigo = self.enigo.lock().map_err(|e| Error::from_reason(format!("Lock error: {}", e)))?;
        let _ = enigo.move_mouse(x, y, enigo::Coordinate::Abs);
        self.apply_delay();
        Ok(())
    }

    /// Move the mouse smoothly to the specified coordinates
    #[napi]
    pub fn move_mouse_smooth(&self, x: i32, y: i32) -> Result<()> {
        self.move_mouse_smooth_with_speed(x, y, 3.0)
    }

    /// Move the mouse smoothly with custom speed
    #[napi]
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
    #[napi]
    pub fn get_mouse_pos(&self) -> Result<MousePosition> {
        let enigo = self.enigo.lock().map_err(|e| Error::from_reason(format!("Lock error: {}", e)))?;
        let (x, y) = enigo.location()
            .map_err(|e| Error::from_reason(format!("Failed to get mouse position: {}", e)))?;
        Ok(MousePosition { x, y })
    }

    /// Click the mouse button
    #[napi]
    pub fn mouse_click(&self, button: Option<String>, double: Option<bool>) -> Result<()> {
        let button_str = button.as_deref().unwrap_or("left");
        let is_double = double.unwrap_or(false);
        
        let mut enigo = self.enigo.lock().map_err(|e| Error::from_reason(format!("Lock error: {}", e)))?;
        
        let mouse_button = match button_str {
            "left" => Button::Left,
            "right" => Button::Right,
            "middle" => Button::Middle,
            _ => return Err(Error::from_reason(format!("Invalid button: {}", button_str))),
        };

        if is_double {
            let _ = enigo.button(mouse_button, Direction::Click);
            thread::sleep(Duration::from_millis(50));
            let _ = enigo.button(mouse_button, Direction::Click);
        } else {
            let _ = enigo.button(mouse_button, Direction::Click);
        }
        
        self.apply_delay();
        Ok(())
    }

    /// Toggle mouse button (press or release)
    #[napi]
    pub fn mouse_toggle(&self, down: String, button: Option<String>) -> Result<()> {
        let button_str = button.as_deref().unwrap_or("left");
        let mut enigo = self.enigo.lock().map_err(|e| Error::from_reason(format!("Lock error: {}", e)))?;
        
        let mouse_button = match button_str {
            "left" => Button::Left,
            "right" => Button::Right,
            "middle" => Button::Middle,
            _ => return Err(Error::from_reason(format!("Invalid button: {}", button_str))),
        };

        let direction = match down.as_str() {
            "down" => Direction::Press,
            "up" => Direction::Release,
            _ => return Err(Error::from_reason(format!("Invalid direction: {}", down))),
        };

        let _ = enigo.button(mouse_button, direction);
        self.apply_delay();
        Ok(())
    }

    /// Drag the mouse to the specified coordinates
    #[napi]
    pub fn drag_mouse(&self, x: i32, y: i32) -> Result<()> {
        // Drag is implemented as mouse down, move, mouse up
        let mut enigo = self.enigo.lock().map_err(|e| Error::from_reason(format!("Lock error: {}", e)))?;
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
    #[napi]
    pub fn scroll_mouse(&self, x: i32, y: i32) -> Result<()> {
        let mut enigo = self.enigo.lock().map_err(|e| Error::from_reason(format!("Lock error: {}", e)))?;
        if x != 0 {
            let _ = enigo.scroll(x, Axis::Horizontal);
        }
        if y != 0 {
            let _ = enigo.scroll(y, Axis::Vertical);
        }
        self.apply_delay();
        Ok(())
    }

    /// Set the mouse delay in milliseconds
    #[napi]
    pub fn set_mouse_delay(&self, delay_ms: u32) -> Result<()> {
        let mut delay = self.delay_ms.lock().map_err(|e| Error::from_reason(format!("Lock error: {}", e)))?;
        *delay = delay_ms;
        Ok(())
    }

    fn apply_delay(&self) {
        let delay = self.delay_ms.lock().ok().map(|d| *d).unwrap_or(10);
        if delay > 0 {
            thread::sleep(Duration::from_millis(delay as u64));
        }
    }
}

#[napi(object)]
pub struct MousePosition {
    pub x: i32,
    pub y: i32,
}

