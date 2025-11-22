use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::sync::{Arc, Mutex};
use crate::screen;
use crate::keyboard;
use crate::mouse;

// Global delay settings
static KEYBOARD_DELAY: once_cell::sync::Lazy<Arc<Mutex<u32>>> = 
    once_cell::sync::Lazy::new(|| Arc::new(Mutex::new(10)));
static MOUSE_DELAY: once_cell::sync::Lazy<Arc<Mutex<u32>>> = 
    once_cell::sync::Lazy::new(|| Arc::new(Mutex::new(10)));

/// Bitmap structure for screen capture
#[napi(object)]
pub struct Bitmap {
    pub width: u32,
    pub height: u32,
    pub image: Buffer,
    pub byte_width: u32,
    pub bits_per_pixel: u32,
    pub bytes_per_pixel: u32,
}

/// Get color at specific coordinates in a bitmap (returns hex string like "#RRGGBB")
#[napi]
pub fn bitmap_color_at(bitmap: Bitmap, x: u32, y: u32) -> Result<String> {
    if x >= bitmap.width || y >= bitmap.height {
        return Err(Error::from_reason("Coordinates out of bounds"));
    }
    // Decode PNG buffer to get pixel color
    // For now, return placeholder - would need to decode PNG buffer
    // This is a simplified implementation
    Ok(format!("#000000"))
}

/// Screen capture interface
#[napi]
pub struct Screen;

#[napi]
impl Screen {
    #[napi(constructor)]
    pub fn new() -> Self {
        Screen
    }

    /// Capture screen region
    #[napi]
    pub async fn capture(
        &self,
        x: Option<u32>,
        y: Option<u32>,
        width: Option<u32>,
        height: Option<u32>,
    ) -> Result<Bitmap> {
        let capture = screen::capture_screen_region(x, y, width, height).await?;
        Ok(Bitmap {
            width: capture.width,
            height: capture.height,
            image: capture.image,
            byte_width: capture.width * 4, // RGBA = 4 bytes per pixel
            bits_per_pixel: 32,
            bytes_per_pixel: 4,
        })
    }
}

/// Get global screen instance (exported as screen variable)
#[napi]
pub fn get_screen() -> Screen {
    Screen::new()
}

/// Set keyboard delay
#[napi]
pub fn set_keyboard_delay(ms: u32) {
    let mut delay = KEYBOARD_DELAY.lock().unwrap();
    *delay = ms;
}

/// Tap a key
#[napi]
pub fn key_tap(key: String, modifier: Option<Vec<String>>) -> Result<()> {
    let keyboard = keyboard::Keyboard::new()?;
    keyboard.key_tap(key, modifier)
}

/// Toggle a key
#[napi]
pub fn key_toggle(key: String, down: String, modifier: Option<Vec<String>>) -> Result<()> {
    let keyboard = keyboard::Keyboard::new()?;
    keyboard.key_toggle(key, down, modifier)
}

/// Tap a Unicode character
#[napi]
pub fn unicode_tap(value: u32) -> Result<()> {
    let keyboard = keyboard::Keyboard::new()?;
    if let Some(ch) = std::char::from_u32(value) {
        keyboard.type_string(ch.to_string())
    } else {
        Err(Error::from_reason(format!("Invalid Unicode value: {}", value)))
    }
}

/// Type a string
#[napi]
pub fn type_string(string: String) -> Result<()> {
    let keyboard = keyboard::Keyboard::new()?;
    keyboard.type_string(string)
}

/// Type a string with delay
#[napi]
pub fn type_string_delayed(string: String, cpm: u32) -> Result<()> {
    let keyboard = keyboard::Keyboard::new()?;
    keyboard.type_string_delayed(string, cpm)
}

/// Set mouse delay
#[napi]
pub fn set_mouse_delay(delay: u32) {
    let mut delay_guard = MOUSE_DELAY.lock().unwrap();
    *delay_guard = delay;
}

/// Update screen metrics (no-op for now, can be used to refresh screen info)
#[napi]
pub fn update_screen_metrics() -> Result<()> {
    // This is a no-op in our implementation
    // Could be used to refresh monitor information if needed
    Ok(())
}

/// Move mouse
#[napi]
pub fn move_mouse(x: i32, y: i32) -> Result<()> {
    let mouse = mouse::Mouse::new()?;
    mouse.move_mouse(x, y)
}

/// Move mouse smoothly
#[napi]
pub fn move_mouse_smooth(x: i32, y: i32, speed: Option<f64>) -> Result<()> {
    let mouse = mouse::Mouse::new()?;
    if let Some(s) = speed {
        mouse.move_mouse_smooth_with_speed(x, y, s)
    } else {
        mouse.move_mouse_smooth(x, y)
    }
}

/// Mouse click
#[napi]
pub fn mouse_click(button: Option<String>, double: Option<bool>) -> Result<()> {
    let mouse = mouse::Mouse::new()?;
    mouse.mouse_click(button, double)
}

/// Mouse toggle
#[napi]
pub fn mouse_toggle(down: Option<String>, button: Option<String>) -> Result<()> {
    let mouse = mouse::Mouse::new()?;
    let down_str = down.unwrap_or_else(|| "down".to_string());
    mouse.mouse_toggle(down_str, button)
}

/// Drag mouse
#[napi]
pub fn drag_mouse(x: i32, y: i32) -> Result<()> {
    let mouse = mouse::Mouse::new()?;
    mouse.drag_mouse(x, y)
}

/// Scroll mouse
#[napi]
pub fn scroll_mouse(x: i32, y: i32) -> Result<()> {
    let mouse = mouse::Mouse::new()?;
    mouse.scroll_mouse(x, y)
}

/// Get mouse position
#[napi]
pub fn get_mouse_pos() -> Result<mouse::MousePosition> {
    let mouse = mouse::Mouse::new()?;
    mouse.get_mouse_pos()
}

/// Get pixel color (returns hex string)
#[napi]
pub async fn get_pixel_color(x: u32, y: u32) -> Result<String> {
    let color = screen::get_pixel_color(x, y).await?;
    Ok(format!("#{:02x}{:02x}{:02x}", color.r, color.g, color.b))
}

/// Get screen size
#[napi]
pub fn get_screen_size() -> Result<screen::ScreenSize> {
    screen::get_screen_size()
}
