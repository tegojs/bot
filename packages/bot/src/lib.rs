//! Bot - N-API bindings for aumate desktop automation library
//!
//! This crate provides Node.js bindings for the aumate library,
//! exposing mouse, keyboard, screen, clipboard, and window functionality.

extern crate napi_derive;

use aumate::prelude::{AumateError, Keyboard, Mouse, WindowInfo, get_active_window_info};
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::sync::{Arc, Mutex};

// Global delay settings
static KEYBOARD_DELAY: once_cell::sync::Lazy<Arc<Mutex<u32>>> =
    once_cell::sync::Lazy::new(|| Arc::new(Mutex::new(10)));
static MOUSE_DELAY: once_cell::sync::Lazy<Arc<Mutex<u32>>> =
    once_cell::sync::Lazy::new(|| Arc::new(Mutex::new(10)));

// ============================================================================
// Type Conversions
// ============================================================================

fn aumate_to_napi_error(e: AumateError) -> Error {
    Error::from_reason(e.to_string())
}

// ============================================================================
// Screen Capture Types
// ============================================================================

/// Screen capture result containing image data
#[napi(object)]
pub struct ScreenCaptureResult {
    pub width: u32,
    pub height: u32,
    pub image: Buffer,
}

/// Screen size information
#[napi(object)]
pub struct ScreenSizeResult {
    pub width: u32,
    pub height: u32,
}

/// Pixel color information
#[napi(object)]
pub struct PixelColorResult {
    pub r: u32,
    pub g: u32,
    pub b: u32,
    pub a: u32,
}

/// Bitmap structure for screen capture (robotjs compatible)
#[napi(object)]
pub struct Bitmap {
    pub width: u32,
    pub height: u32,
    pub image: Buffer,
    pub byte_width: u32,
    pub bits_per_pixel: u32,
    pub bytes_per_pixel: u32,
}

/// Mouse position
#[napi(object)]
pub struct MousePositionResult {
    pub x: i32,
    pub y: i32,
}

/// Window information structure
#[napi(object)]
pub struct WindowInfoResult {
    pub title: String,
    pub process_id: u32,
    pub process_path: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub window_id: String,
}

impl From<WindowInfo> for WindowInfoResult {
    fn from(info: WindowInfo) -> Self {
        Self {
            title: info.title,
            process_id: info.process_id,
            process_path: info.process_path,
            x: info.x,
            y: info.y,
            width: info.width,
            height: info.height,
            window_id: info.window_id,
        }
    }
}

// ============================================================================
// Screen Interface
// ============================================================================

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
        let capture = aumate::prelude::capture_screen_region(x, y, width, height)
            .map_err(aumate_to_napi_error)?;
        Ok(Bitmap {
            width: capture.width,
            height: capture.height,
            image: Buffer::from(capture.image),
            byte_width: capture.width * 4,
            bits_per_pixel: 32,
            bytes_per_pixel: 4,
        })
    }
}

impl Default for Screen {
    fn default() -> Self {
        Self::new()
    }
}

/// Get global screen instance
#[napi]
pub fn get_screen() -> Screen {
    Screen::new()
}

/// Get color at specific coordinates in a bitmap
#[napi]
pub fn bitmap_color_at(bitmap: Bitmap, x: u32, y: u32) -> Result<String> {
    if x >= bitmap.width || y >= bitmap.height {
        return Err(Error::from_reason("Coordinates out of bounds"));
    }
    // Return placeholder - would need to decode PNG buffer
    Ok("#000000".to_string())
}

// ============================================================================
// Keyboard Operations
// ============================================================================

/// Set keyboard delay
#[napi]
pub fn set_keyboard_delay(ms: u32) {
    if let Ok(mut delay) = KEYBOARD_DELAY.lock() {
        *delay = ms;
    }
}

/// Tap a key
#[napi]
pub fn key_tap(key: String, modifier: Option<Vec<String>>) -> Result<()> {
    let keyboard = Keyboard::new().map_err(aumate_to_napi_error)?;
    keyboard.key_tap(&key, modifier.as_deref()).map_err(aumate_to_napi_error)
}

/// Toggle a key
#[napi]
pub fn key_toggle(key: String, down: String, modifier: Option<Vec<String>>) -> Result<()> {
    let keyboard = Keyboard::new().map_err(aumate_to_napi_error)?;
    keyboard.key_toggle(&key, &down, modifier.as_deref()).map_err(aumate_to_napi_error)
}

/// Tap a Unicode character
#[napi]
pub fn unicode_tap(value: u32) -> Result<()> {
    let keyboard = Keyboard::new().map_err(aumate_to_napi_error)?;
    if let Some(ch) = std::char::from_u32(value) {
        keyboard.unicode_tap(ch).map_err(aumate_to_napi_error)
    } else {
        Err(Error::from_reason(format!("Invalid Unicode value: {}", value)))
    }
}

/// Type a string
#[napi]
pub fn type_string(string: String) -> Result<()> {
    let keyboard = Keyboard::new().map_err(aumate_to_napi_error)?;
    keyboard.type_string(&string).map_err(aumate_to_napi_error)
}

/// Type a string with delay
#[napi]
pub fn type_string_delayed(string: String, cpm: u32) -> Result<()> {
    let keyboard = Keyboard::new().map_err(aumate_to_napi_error)?;
    keyboard.type_string_delayed(&string, cpm).map_err(aumate_to_napi_error)
}

// ============================================================================
// Mouse Operations
// ============================================================================

/// Set mouse delay
#[napi]
pub fn set_mouse_delay(delay: u32) {
    if let Ok(mut delay_guard) = MOUSE_DELAY.lock() {
        *delay_guard = delay;
    }
}

/// Update screen metrics (no-op for now)
#[napi]
pub fn update_screen_metrics() -> Result<()> {
    Ok(())
}

/// Move mouse
#[napi]
pub fn move_mouse(x: i32, y: i32) -> Result<()> {
    let mouse = Mouse::new().map_err(aumate_to_napi_error)?;
    mouse.move_mouse(x, y).map_err(aumate_to_napi_error)
}

/// Move mouse smoothly
#[napi]
pub fn move_mouse_smooth(x: i32, y: i32, speed: Option<f64>) -> Result<()> {
    let mouse = Mouse::new().map_err(aumate_to_napi_error)?;
    if let Some(s) = speed {
        mouse.move_mouse_smooth_with_speed(x, y, s).map_err(aumate_to_napi_error)
    } else {
        mouse.move_mouse_smooth(x, y).map_err(aumate_to_napi_error)
    }
}

/// Mouse click
#[napi]
pub fn mouse_click(button: Option<String>, double: Option<bool>) -> Result<()> {
    let mouse = Mouse::new().map_err(aumate_to_napi_error)?;
    mouse.mouse_click(button.as_deref(), double).map_err(aumate_to_napi_error)
}

/// Mouse toggle
#[napi]
pub fn mouse_toggle(down: Option<String>, button: Option<String>) -> Result<()> {
    let mouse = Mouse::new().map_err(aumate_to_napi_error)?;
    let down_str = down.unwrap_or_else(|| "down".to_string());
    mouse.mouse_toggle(&down_str, button.as_deref()).map_err(aumate_to_napi_error)
}

/// Drag mouse
#[napi]
pub fn drag_mouse(x: i32, y: i32) -> Result<()> {
    let mouse = Mouse::new().map_err(aumate_to_napi_error)?;
    mouse.drag_mouse(x, y).map_err(aumate_to_napi_error)
}

/// Scroll mouse
#[napi]
pub fn scroll_mouse(x: i32, y: i32) -> Result<()> {
    let mouse = Mouse::new().map_err(aumate_to_napi_error)?;
    mouse.scroll_mouse(x, y).map_err(aumate_to_napi_error)
}

/// Get mouse position
#[napi]
pub fn get_mouse_pos() -> Result<MousePositionResult> {
    let mouse = Mouse::new().map_err(aumate_to_napi_error)?;
    let pos = mouse.get_mouse_pos().map_err(aumate_to_napi_error)?;
    Ok(MousePositionResult { x: pos.x, y: pos.y })
}

// ============================================================================
// Screen Capture Operations
// ============================================================================

/// Get pixel color (returns hex string)
#[napi]
pub async fn get_pixel_color(x: u32, y: u32) -> Result<String> {
    let color = aumate::screen::get_pixel_color(x, y).map_err(aumate_to_napi_error)?;
    Ok(format!("#{:02x}{:02x}{:02x}", color.r, color.g, color.b))
}

/// Get screen size
#[napi]
pub fn get_screen_size() -> Result<ScreenSizeResult> {
    let size = aumate::screen::get_screen_size().map_err(aumate_to_napi_error)?;
    Ok(ScreenSizeResult { width: size.width, height: size.height })
}

/// Capture entire screen
#[napi]
pub async fn capture_screen() -> Result<ScreenCaptureResult> {
    let capture = aumate::screen::capture_screen().map_err(aumate_to_napi_error)?;
    Ok(ScreenCaptureResult {
        width: capture.width,
        height: capture.height,
        image: Buffer::from(capture.image),
    })
}

/// Capture screen region
#[napi]
pub async fn capture_screen_region(
    x: u32,
    y: u32,
    width: u32,
    height: u32,
) -> Result<ScreenCaptureResult> {
    let capture =
        aumate::screen::capture_screen_region(Some(x), Some(y), Some(width), Some(height))
            .map_err(aumate_to_napi_error)?;
    Ok(ScreenCaptureResult {
        width: capture.width,
        height: capture.height,
        image: Buffer::from(capture.image),
    })
}

// ============================================================================
// Clipboard Operations
// ============================================================================

/// Get text from clipboard
#[napi]
pub fn get_clipboard() -> Result<String> {
    aumate::clipboard::get_text().map_err(aumate_to_napi_error)
}

/// Set text to clipboard
#[napi]
pub fn set_clipboard(text: String) -> Result<()> {
    aumate::clipboard::set_text(&text).map_err(aumate_to_napi_error)
}

/// Get image from clipboard (returns PNG-encoded buffer)
#[napi]
pub fn get_clipboard_image() -> Result<Buffer> {
    let image = aumate::clipboard::get_image().map_err(aumate_to_napi_error)?;
    Ok(Buffer::from(image))
}

/// Set image to clipboard (accepts PNG-encoded buffer)
#[napi]
pub fn set_clipboard_image(image_buffer: Buffer) -> Result<()> {
    aumate::clipboard::set_image(&image_buffer).map_err(aumate_to_napi_error)
}

/// Clear clipboard
#[napi]
pub fn clear_clipboard() -> Result<()> {
    aumate::clipboard::clear().map_err(aumate_to_napi_error)
}

// ============================================================================
// Window Management
// ============================================================================

/// Get the currently active (focused) window
#[napi]
pub fn get_active_window() -> Result<WindowInfoResult> {
    let info = get_active_window_info().map_err(aumate_to_napi_error)?;
    Ok(info.into())
}

/// Get a list of all visible windows
#[napi]
pub fn get_all_windows() -> Result<Vec<WindowInfoResult>> {
    let windows = aumate::prelude::get_all_windows().map_err(aumate_to_napi_error)?;
    Ok(windows.into_iter().map(|w| w.into()).collect())
}

/// Find windows by title (case-insensitive partial match)
#[napi]
pub fn find_windows_by_title(title: String) -> Result<Vec<WindowInfoResult>> {
    let windows = aumate::prelude::find_windows_by_title(&title).map_err(aumate_to_napi_error)?;
    Ok(windows.into_iter().map(|w| w.into()).collect())
}

/// Find windows by process name (case-insensitive partial match)
#[napi]
pub fn find_windows_by_process(process_name: String) -> Result<Vec<WindowInfoResult>> {
    let windows =
        aumate::prelude::find_windows_by_process(&process_name).map_err(aumate_to_napi_error)?;
    Ok(windows.into_iter().map(|w| w.into()).collect())
}
