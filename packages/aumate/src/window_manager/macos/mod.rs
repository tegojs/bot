//! macOS window manipulation using AXUIElement API
//!
//! This module provides functions to move and resize windows on macOS
//! using the Accessibility APIs (AXUIElement).

#![allow(unexpected_cfgs)]

use crate::error::{AumateError, Result};
use objc::runtime::{BOOL, Class, NO, Object};
use objc::{msg_send, sel, sel_impl};
use std::ffi::c_void;
use std::ptr;

// Core Foundation types
type CFStringRef = *const c_void;
type CFTypeRef = *const c_void;
type CFArrayRef = *const c_void;
type CFIndex = isize;

// AXUIElement types
type AXUIElementRef = *const c_void;
type AXValueRef = *const c_void;
type AXError = i32;

const AX_ERROR_SUCCESS: AXError = 0;

// AXValue types
const AX_VALUE_TYPE_CGPOINT: i32 = 1;
const AX_VALUE_TYPE_CGSIZE: i32 = 2;

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
struct CGPoint {
    x: f64,
    y: f64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
struct CGSize {
    width: f64,
    height: f64,
}

#[link(name = "ApplicationServices", kind = "framework")]
unsafe extern "C" {
    fn AXUIElementCreateApplication(pid: i32) -> AXUIElementRef;
    fn AXUIElementCopyAttributeValue(
        element: AXUIElementRef,
        attribute: CFStringRef,
        value: *mut CFTypeRef,
    ) -> AXError;
    fn AXUIElementSetAttributeValue(
        element: AXUIElementRef,
        attribute: CFStringRef,
        value: CFTypeRef,
    ) -> AXError;
    fn AXIsProcessTrusted() -> BOOL;
    fn AXValueGetValue(value: AXValueRef, value_type: i32, value_ptr: *mut c_void) -> bool;
    fn AXValueCreate(value_type: i32, value_ptr: *const c_void) -> AXValueRef;
}

#[link(name = "CoreFoundation", kind = "framework")]
unsafe extern "C" {
    fn CFStringCreateWithCString(
        alloc: *const c_void,
        cstr: *const i8,
        encoding: u32,
    ) -> CFStringRef;
    fn CFArrayGetCount(array: CFArrayRef) -> CFIndex;
    fn CFArrayGetValueAtIndex(array: CFArrayRef, idx: CFIndex) -> CFTypeRef;
    fn CFRelease(cf: CFTypeRef);
}

const K_CF_STRING_ENCODING_UTF8: u32 = 0x08000100;

// Accessibility attribute names
static K_AX_WINDOWS_ATTRIBUTE: &[u8] = b"AXWindows\0";
static K_AX_POSITION_ATTRIBUTE: &[u8] = b"AXPosition\0";
static K_AX_SIZE_ATTRIBUTE: &[u8] = b"AXSize\0";
static K_AX_FOCUSED_WINDOW_ATTRIBUTE: &[u8] = b"AXFocusedWindow\0";
static K_AX_MAIN_WINDOW_ATTRIBUTE: &[u8] = b"AXMainWindow\0";

fn create_cf_string(s: &[u8]) -> CFStringRef {
    unsafe {
        CFStringCreateWithCString(ptr::null(), s.as_ptr() as *const i8, K_CF_STRING_ENCODING_UTF8)
    }
}

/// Check if accessibility permission is granted
pub fn is_accessibility_trusted() -> bool {
    unsafe { AXIsProcessTrusted() != NO }
}

/// Get the frontmost application PID, excluding the specified PID
pub fn get_frontmost_app_pid_excluding(exclude_pid: i32) -> Option<i32> {
    unsafe {
        let workspace_class = Class::get("NSWorkspace")?;
        let workspace: *mut Object = msg_send![workspace_class, sharedWorkspace];
        if workspace.is_null() {
            return None;
        }

        // First try the frontmost app
        let frontmost_app: *mut Object = msg_send![workspace, frontmostApplication];
        if !frontmost_app.is_null() {
            let pid: i32 = msg_send![frontmost_app, processIdentifier];
            if pid != exclude_pid {
                return Some(pid);
            }
        }

        // If frontmost is ourselves, find the next app in the list
        let running_apps: *mut Object = msg_send![workspace, runningApplications];
        if running_apps.is_null() {
            return None;
        }

        let count: usize = msg_send![running_apps, count];
        for i in 0..count {
            let app: *mut Object = msg_send![running_apps, objectAtIndex: i];
            if app.is_null() {
                continue;
            }

            let activation_policy: i64 = msg_send![app, activationPolicy];
            if activation_policy != 0 {
                continue; // Skip non-regular apps
            }

            let pid: i32 = msg_send![app, processIdentifier];
            if pid == exclude_pid {
                continue;
            }

            let is_hidden: bool = msg_send![app, isHidden];
            if !is_hidden {
                return Some(pid);
            }
        }

        None
    }
}

/// Get the frontmost application PID
pub fn get_frontmost_app_pid() -> Option<i32> {
    get_frontmost_app_pid_excluding(-1)
}

/// Get the focused window element for an application
fn get_focused_window(app_element: AXUIElementRef) -> Option<AXUIElementRef> {
    unsafe {
        // Try focused window first
        let attr_name = create_cf_string(K_AX_FOCUSED_WINDOW_ATTRIBUTE);
        let mut value: CFTypeRef = ptr::null();
        let err = AXUIElementCopyAttributeValue(app_element, attr_name, &mut value);
        CFRelease(attr_name as CFTypeRef);

        if err == AX_ERROR_SUCCESS && !value.is_null() {
            return Some(value as AXUIElementRef);
        }

        // Fall back to main window
        let attr_name = create_cf_string(K_AX_MAIN_WINDOW_ATTRIBUTE);
        let mut value: CFTypeRef = ptr::null();
        let err = AXUIElementCopyAttributeValue(app_element, attr_name, &mut value);
        CFRelease(attr_name as CFTypeRef);

        if err == AX_ERROR_SUCCESS && !value.is_null() {
            return Some(value as AXUIElementRef);
        }

        // Fall back to first window
        let attr_name = create_cf_string(K_AX_WINDOWS_ATTRIBUTE);
        let mut value: CFTypeRef = ptr::null();
        let err = AXUIElementCopyAttributeValue(app_element, attr_name, &mut value);
        CFRelease(attr_name as CFTypeRef);

        if err != AX_ERROR_SUCCESS || value.is_null() {
            return None;
        }

        let count = CFArrayGetCount(value as CFArrayRef);
        if count > 0 {
            let window = CFArrayGetValueAtIndex(value as CFArrayRef, 0);
            // Don't release the array yet - we're returning a reference to its element
            // The caller must be careful about this
            Some(window as AXUIElementRef)
        } else {
            CFRelease(value);
            None
        }
    }
}

/// Get current window position for an app
pub fn get_window_position(pid: i32) -> Result<(f64, f64)> {
    if !is_accessibility_trusted() {
        return Err(AumateError::Other("Accessibility permission not granted".to_string()));
    }

    unsafe {
        let app = AXUIElementCreateApplication(pid);
        if app.is_null() {
            return Err(AumateError::Other("Failed to create AXUIElement".to_string()));
        }

        let window = get_focused_window(app).ok_or_else(|| {
            CFRelease(app as CFTypeRef);
            AumateError::Other("No focused window found".to_string())
        })?;

        let attr_name = create_cf_string(K_AX_POSITION_ATTRIBUTE);
        let mut value: CFTypeRef = ptr::null();
        let err = AXUIElementCopyAttributeValue(window, attr_name, &mut value);
        CFRelease(attr_name as CFTypeRef);

        if err != AX_ERROR_SUCCESS || value.is_null() {
            CFRelease(app as CFTypeRef);
            return Err(AumateError::Other("Failed to get window position".to_string()));
        }

        let mut point = CGPoint::default();
        let success = AXValueGetValue(
            value as AXValueRef,
            AX_VALUE_TYPE_CGPOINT,
            &mut point as *mut _ as *mut c_void,
        );
        CFRelease(value);
        CFRelease(app as CFTypeRef);

        if success {
            Ok((point.x, point.y))
        } else {
            Err(AumateError::Other("Failed to extract position value".to_string()))
        }
    }
}

/// Get current window size for an app
pub fn get_window_size(pid: i32) -> Result<(f64, f64)> {
    if !is_accessibility_trusted() {
        return Err(AumateError::Other("Accessibility permission not granted".to_string()));
    }

    unsafe {
        let app = AXUIElementCreateApplication(pid);
        if app.is_null() {
            return Err(AumateError::Other("Failed to create AXUIElement".to_string()));
        }

        let window = get_focused_window(app).ok_or_else(|| {
            CFRelease(app as CFTypeRef);
            AumateError::Other("No focused window found".to_string())
        })?;

        let attr_name = create_cf_string(K_AX_SIZE_ATTRIBUTE);
        let mut value: CFTypeRef = ptr::null();
        let err = AXUIElementCopyAttributeValue(window, attr_name, &mut value);
        CFRelease(attr_name as CFTypeRef);

        if err != AX_ERROR_SUCCESS || value.is_null() {
            CFRelease(app as CFTypeRef);
            return Err(AumateError::Other("Failed to get window size".to_string()));
        }

        let mut size = CGSize::default();
        let success = AXValueGetValue(
            value as AXValueRef,
            AX_VALUE_TYPE_CGSIZE,
            &mut size as *mut _ as *mut c_void,
        );
        CFRelease(value);
        CFRelease(app as CFTypeRef);

        if success {
            Ok((size.width, size.height))
        } else {
            Err(AumateError::Other("Failed to extract size value".to_string()))
        }
    }
}

/// Set window position for an app's focused window
pub fn set_window_position(pid: i32, x: f64, y: f64) -> Result<()> {
    if !is_accessibility_trusted() {
        return Err(AumateError::Other("Accessibility permission not granted".to_string()));
    }

    unsafe {
        let app = AXUIElementCreateApplication(pid);
        if app.is_null() {
            return Err(AumateError::Other("Failed to create AXUIElement".to_string()));
        }

        let window = get_focused_window(app).ok_or_else(|| {
            CFRelease(app as CFTypeRef);
            AumateError::Other("No focused window found".to_string())
        })?;

        let point = CGPoint { x, y };
        let value = AXValueCreate(AX_VALUE_TYPE_CGPOINT, &point as *const _ as *const c_void);
        if value.is_null() {
            CFRelease(app as CFTypeRef);
            return Err(AumateError::Other("Failed to create AXValue".to_string()));
        }

        let attr_name = create_cf_string(K_AX_POSITION_ATTRIBUTE);
        let err = AXUIElementSetAttributeValue(window, attr_name, value as CFTypeRef);
        CFRelease(attr_name as CFTypeRef);
        CFRelease(value as CFTypeRef);
        CFRelease(app as CFTypeRef);

        if err == AX_ERROR_SUCCESS {
            Ok(())
        } else {
            Err(AumateError::Other(format!("Failed to set window position: AXError {}", err)))
        }
    }
}

/// Set window size for an app's focused window
pub fn set_window_size(pid: i32, width: f64, height: f64) -> Result<()> {
    if !is_accessibility_trusted() {
        return Err(AumateError::Other("Accessibility permission not granted".to_string()));
    }

    unsafe {
        let app = AXUIElementCreateApplication(pid);
        if app.is_null() {
            return Err(AumateError::Other("Failed to create AXUIElement".to_string()));
        }

        let window = get_focused_window(app).ok_or_else(|| {
            CFRelease(app as CFTypeRef);
            AumateError::Other("No focused window found".to_string())
        })?;

        let size = CGSize { width, height };
        let value = AXValueCreate(AX_VALUE_TYPE_CGSIZE, &size as *const _ as *const c_void);
        if value.is_null() {
            CFRelease(app as CFTypeRef);
            return Err(AumateError::Other("Failed to create AXValue".to_string()));
        }

        let attr_name = create_cf_string(K_AX_SIZE_ATTRIBUTE);
        let err = AXUIElementSetAttributeValue(window, attr_name, value as CFTypeRef);
        CFRelease(attr_name as CFTypeRef);
        CFRelease(value as CFTypeRef);
        CFRelease(app as CFTypeRef);

        if err == AX_ERROR_SUCCESS {
            Ok(())
        } else {
            Err(AumateError::Other(format!("Failed to set window size: AXError {}", err)))
        }
    }
}

/// Set window frame (position and size) for an app's focused window
pub fn set_window_frame(pid: i32, x: f64, y: f64, width: f64, height: f64) -> Result<()> {
    // Set position first, then size
    // This order is important because some windows have minimum size constraints
    set_window_position(pid, x, y)?;
    set_window_size(pid, width, height)?;
    Ok(())
}

/// Window bounds structure
#[derive(Debug, Clone, Copy)]
pub struct WindowBounds {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

/// Get the current window bounds (position and size)
pub fn get_window_bounds(pid: i32) -> Result<WindowBounds> {
    let (x, y) = get_window_position(pid)?;
    let (width, height) = get_window_size(pid)?;
    Ok(WindowBounds { x, y, width, height })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_accessibility_trusted() {
        // This will return true or false depending on system permissions
        let _ = is_accessibility_trusted();
    }

    #[test]
    fn test_get_frontmost_app_pid() {
        // This should return Some pid if there's an active app
        let pid = get_frontmost_app_pid();
        // Can't assert specific value as it depends on running system
        println!("Frontmost app PID: {:?}", pid);
    }
}
