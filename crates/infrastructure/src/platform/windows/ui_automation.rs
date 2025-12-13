#![cfg(target_os = "windows")]

use std::sync::Arc;

use aumate_core_shared::Rectangle;
use aumate_core_traits::window::UIElement;
use uiautomation::UIAutomation;
use uiautomation::UITreeWalker;
use uiautomation::core::UICacheRequest;
use uiautomation::types::{Point, TreeScope, UIProperty};
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{
    IsIconic, PostMessageW, SW_RESTORE, SetForegroundWindow, ShowWindow, WM_CLOSE,
};
use xcap::Window;

/// Window element information
#[derive(Debug, Clone)]
pub struct WindowElement {
    pub rect: Rectangle,
    pub window_id: u32,
    pub title: String,
    pub app_name: String,
}

/// Wrapper for UIAutomation to make it Send + Sync
struct UIAutomationWrapper {
    automation: UIAutomation,
}

unsafe impl Send for UIAutomationWrapper {}
unsafe impl Sync for UIAutomationWrapper {}

/// UI Elements manager for Windows
pub struct UIElements {
    automation: Option<Arc<UIAutomationWrapper>>,
    #[allow(dead_code)]
    automation_walker: Option<UITreeWalker>,
    cache_request: Option<UICacheRequest>,
}

unsafe impl Send for UIElements {}
unsafe impl Sync for UIElements {}

impl UIElements {
    pub fn new() -> Self {
        Self { automation: None, automation_walker: None, cache_request: None }
    }

    /// Initialize the UI automation
    pub fn init(&mut self) -> Result<(), String> {
        if self.automation.is_some() {
            return Ok(());
        }

        let automation =
            UIAutomation::new().map_err(|e| format!("Failed to create UIAutomation: {}", e))?;

        let walker = automation
            .get_content_view_walker()
            .map_err(|e| format!("Failed to get tree walker: {}", e))?;

        // Create cache request for performance
        let cache_request = automation
            .create_cache_request()
            .map_err(|e| format!("Failed to create cache request: {}", e))?;

        cache_request
            .add_property(UIProperty::BoundingRectangle)
            .map_err(|e| format!("Failed to add property: {}", e))?;
        cache_request
            .add_property(UIProperty::ControlType)
            .map_err(|e| format!("Failed to add property: {}", e))?;
        cache_request
            .add_property(UIProperty::IsOffscreen)
            .map_err(|e| format!("Failed to add property: {}", e))?;
        cache_request
            .set_tree_scope(TreeScope::Element)
            .map_err(|e| format!("Failed to set tree scope: {}", e))?;

        self.automation = Some(Arc::new(UIAutomationWrapper { automation }));
        self.automation_walker = Some(walker);
        self.cache_request = Some(cache_request);

        Ok(())
    }

    pub fn init_cache(&mut self) -> Result<(), String> {
        // Cache initialization handled in init()
        Ok(())
    }

    /// Get the element at a specific screen position
    pub fn get_element_at_point(&self, x: i32, y: i32) -> Result<Option<UIElement>, String> {
        let automation = match &self.automation {
            Some(a) => a,
            None => return Err("UIAutomation not initialized".to_string()),
        };

        let element = if let Some(cache) = &self.cache_request {
            automation.automation.element_from_point_build_cache(Point::new(x, y), cache).ok()
        } else {
            automation.automation.element_from_point(Point::new(x, y)).ok()
        };

        if let Some(element) = element {
            let rect = if self.cache_request.is_some() {
                element.get_cached_bounding_rectangle().ok()
            } else {
                element.get_bounding_rectangle().ok()
            };

            if let Some(rect) = rect {
                return Ok(Some(UIElement {
                    bounds: Rectangle::new(
                        rect.get_left(),
                        rect.get_top(),
                        (rect.get_right() - rect.get_left()) as u32,
                        (rect.get_bottom() - rect.get_top()) as u32,
                    ),
                    role: None,
                    title: None,
                    value: None,
                }));
            }
        }

        Ok(None)
    }

    pub fn get_elements_at_position(&self, x: i32, y: i32) -> Result<Vec<UIElement>, String> {
        match self.get_element_at_point(x, y)? {
            Some(elem) => Ok(vec![elem]),
            None => Ok(vec![]),
        }
    }

    pub fn get_window_elements(&self, _window_id: &str) -> Result<Vec<UIElement>, String> {
        // TODO: Implement window element traversal
        Ok(vec![])
    }

    pub fn clear_cache(&self) {
        // Cache clearing not needed for Windows
    }
}

impl Default for UIElements {
    fn default() -> Self {
        Self::new()
    }
}

/// Get all visible windows
pub fn get_all_windows() -> Result<Vec<WindowElement>, String> {
    let windows = Window::all().map_err(|e| format!("Failed to get windows: {}", e))?;

    let mut result = Vec::new();

    for window in windows {
        // Skip minimized windows (handle Result)
        if window.is_minimized().unwrap_or(false) {
            continue;
        }

        let title = window.title().unwrap_or_default();

        // Skip certain system windows
        if title.is_empty()
            || title == "Shell Handwriting Canvas"
            || title == "Program Manager"
            || title.starts_with("MSCTFIME")
        {
            continue;
        }

        // Get window properties, skip if any fail
        let Ok(x) = window.x() else { continue };
        let Ok(y) = window.y() else { continue };
        let Ok(width) = window.width() else { continue };
        let Ok(height) = window.height() else { continue };
        let Ok(window_id) = window.id() else { continue };
        let app_name = window.app_name().unwrap_or_default();

        let rect = Rectangle::new(x, y, width, height);

        // Skip windows with zero size
        if rect.width == 0 || rect.height == 0 {
            continue;
        }

        result.push(WindowElement { rect, window_id, title, app_name });
    }

    // Sort by z-order (windows earlier in the list are on top)
    Ok(result)
}

/// Get the window element at a specific point
pub fn get_window_at_point(x: i32, y: i32) -> Result<Option<WindowElement>, String> {
    let windows = get_all_windows()?;

    for window in windows {
        if x >= window.rect.x
            && x < window.rect.x + window.rect.width as i32
            && y >= window.rect.y
            && y < window.rect.y + window.rect.height as i32
        {
            return Ok(Some(window));
        }
    }

    Ok(None)
}

/// Switch to a window by its ID (HWND)
pub fn switch_to_window(window_id: u32) -> Result<(), String> {
    unsafe {
        let hwnd = HWND(window_id as isize as *mut std::ffi::c_void);

        // Restore if minimized
        if IsIconic(hwnd).as_bool() {
            let _ = ShowWindow(hwnd, SW_RESTORE);
        }

        // Bring to foreground
        if !SetForegroundWindow(hwnd).as_bool() {
            return Err("Failed to set foreground window".to_string());
        }

        Ok(())
    }
}

/// Close a window by its ID (HWND)
pub fn close_window(window_id: u32) -> Result<(), String> {
    use windows::Win32::Foundation::{LPARAM, WPARAM};

    unsafe {
        let hwnd = HWND(window_id as isize as *mut std::ffi::c_void);

        // Send WM_CLOSE message
        PostMessageW(Some(hwnd), WM_CLOSE, WPARAM(0), LPARAM(0))
            .map_err(|e| format!("Failed to close window: {}", e))?;

        Ok(())
    }
}
