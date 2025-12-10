#![cfg(target_os = "windows")]

use std::sync::Arc;

use uiautomation::UIAutomation;
use uiautomation::UITreeWalker;
use uiautomation::core::UICacheRequest;
use uiautomation::types::{Point, TreeScope, UIProperty};
use xcap::Window;

use crate::screenshot::types::{ElementRect, WindowElement};

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
        Self {
            automation: None,
            automation_walker: None,
            cache_request: None,
        }
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

    /// Get the element at a specific screen position
    pub fn get_element_at_point(&self, x: i32, y: i32) -> Result<Option<ElementRect>, String> {
        let automation = match &self.automation {
            Some(a) => a,
            None => return Err("UIAutomation not initialized".to_string()),
        };

        let element = if let Some(cache) = &self.cache_request {
            automation
                .automation
                .element_from_point_build_cache(Point::new(x, y), cache)
                .ok()
        } else {
            automation
                .automation
                .element_from_point(Point::new(x, y))
                .ok()
        };

        if let Some(element) = element {
            let rect = if self.cache_request.is_some() {
                element.get_cached_bounding_rectangle().ok()
            } else {
                element.get_bounding_rectangle().ok()
            };

            if let Some(rect) = rect {
                return Ok(Some(ElementRect::new(
                    rect.get_left(),
                    rect.get_top(),
                    rect.get_right(),
                    rect.get_bottom(),
                )));
            }
        }

        Ok(None)
    }
}

/// Get all visible windows
pub fn get_all_windows() -> Result<Vec<WindowElement>, String> {
    let windows = Window::all().map_err(|e| format!("Failed to get windows: {}", e))?;

    let mut result = Vec::new();

    for window in windows {
        // Skip minimized windows
        if window.is_minimized() {
            continue;
        }

        let title = window.title().to_string();

        // Skip certain system windows
        if title.is_empty()
            || title == "Shell Handwriting Canvas"
            || title == "Program Manager"
            || title.starts_with("MSCTFIME")
        {
            continue;
        }

        let x = window.x();
        let y = window.y();
        let width = window.width() as i32;
        let height = window.height() as i32;

        let rect = ElementRect::new(x, y, x + width, y + height);

        // Skip windows with zero size
        if rect.width() <= 0 || rect.height() <= 0 {
            continue;
        }

        result.push(WindowElement {
            rect,
            window_id: window.id(),
            title,
            app_name: window.app_name().to_string(),
        });
    }

    // Sort by z-order (windows earlier in the list are on top)
    Ok(result)
}

/// Get the window element at a specific point
pub fn get_window_at_point(x: i32, y: i32) -> Result<Option<WindowElement>, String> {
    let windows = get_all_windows()?;

    for window in windows {
        if window.rect.contains_point(x, y) {
            return Ok(Some(window));
        }
    }

    Ok(None)
}
