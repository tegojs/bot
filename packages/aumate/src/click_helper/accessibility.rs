//! Platform-specific accessibility APIs for discovering clickable UI elements
//!
//! macOS: Uses AXUIElement API via the Accessibility framework

use crate::error::{AumateError, Result};

/// Check if Input Monitoring permission is granted (required for global hotkeys on macOS 10.15+)
#[cfg(target_os = "macos")]
pub fn is_input_monitoring_enabled() -> bool {
    #[link(name = "IOKit", kind = "framework")]
    unsafe extern "C" {
        fn IOHIDCheckAccess(requestType: u32) -> u32;
    }

    const K_IOHID_REQUEST_TYPE_LISTEN_EVENT: u32 = 1;

    // IOHIDCheckAccess returns: 0 = granted, 1 = denied, 2 = unknown
    unsafe { IOHIDCheckAccess(K_IOHID_REQUEST_TYPE_LISTEN_EVENT) == 0 }
}

/// Request Input Monitoring permission (opens system dialog on macOS 10.15+)
#[cfg(target_os = "macos")]
pub fn request_input_monitoring_permission() {
    #[link(name = "IOKit", kind = "framework")]
    unsafe extern "C" {
        fn IOHIDRequestAccess(requestType: u32) -> bool;
    }

    const K_IOHID_REQUEST_TYPE_LISTEN_EVENT: u32 = 1;

    unsafe {
        let _ = IOHIDRequestAccess(K_IOHID_REQUEST_TYPE_LISTEN_EVENT);
    }
}

/// Open Input Monitoring settings in System Preferences
#[cfg(target_os = "macos")]
pub fn open_input_monitoring_settings() {
    use std::process::Command;
    let _ = Command::new("open")
        .arg("x-apple.systempreferences:com.apple.preference.security?Privacy_ListenEvent")
        .spawn();
}

#[cfg(not(target_os = "macos"))]
pub fn is_input_monitoring_enabled() -> bool {
    true
}

#[cfg(not(target_os = "macos"))]
pub fn request_input_monitoring_permission() {}

#[cfg(not(target_os = "macos"))]
pub fn open_input_monitoring_settings() {}

/// Represents a clickable UI element discovered via accessibility APIs
#[derive(Debug, Clone)]
pub struct ClickableElement {
    /// Screen position (center point for clicking)
    pub position: (f32, f32),
    /// Bounding rectangle (x, y, width, height)
    pub bounds: (f32, f32, f32, f32),
    /// Element role (Button, Link, MenuItem, etc.)
    pub role: String,
    /// Element name/label (optional)
    pub name: Option<String>,
}

impl ClickableElement {
    /// Get the center point of the element
    pub fn center(&self) -> (f32, f32) {
        (self.bounds.0 + self.bounds.2 / 2.0, self.bounds.1 + self.bounds.3 / 2.0)
    }
}

/// Platform-specific accessibility implementation trait
pub trait AccessibilityProvider: Send + Sync {
    /// Get all clickable elements on screen
    fn get_clickable_elements(&self) -> Result<Vec<ClickableElement>>;

    /// Check if accessibility permissions are granted
    fn is_trusted(&self) -> bool;

    /// Request accessibility permissions (shows system dialog on macOS)
    fn request_permission(&self);
}

// ============================================================================
// macOS Implementation using AXUIElement API
// ============================================================================

#[cfg(target_os = "macos")]
#[allow(unexpected_cfgs)]
mod macos {
    use super::*;
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
        fn AXIsProcessTrusted() -> BOOL;
        fn AXIsProcessTrustedWithOptions(options: CFTypeRef) -> BOOL;
        fn AXValueGetValue(value: AXValueRef, value_type: i32, value_ptr: *mut c_void) -> bool;
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
        fn CFGetTypeID(cf: CFTypeRef) -> u64;
        fn CFStringGetTypeID() -> u64;
        fn CFStringGetCString(
            string: CFStringRef,
            buffer: *mut i8,
            buffer_size: CFIndex,
            encoding: u32,
        ) -> BOOL;
        fn CFDictionaryCreate(
            allocator: *const c_void,
            keys: *const CFTypeRef,
            values: *const CFTypeRef,
            num_values: CFIndex,
            key_callbacks: *const c_void,
            value_callbacks: *const c_void,
        ) -> CFTypeRef;
        fn CFBooleanGetValue(boolean: CFTypeRef) -> BOOL;

        // kCFBooleanTrue is a global constant
        static kCFBooleanTrue: CFTypeRef;
    }

    const K_CF_STRING_ENCODING_UTF8: u32 = 0x08000100;

    // Global constants for accessibility attributes
    static K_AX_WINDOWS_ATTRIBUTE: &[u8] = b"AXWindows\0";
    static K_AX_CHILDREN_ATTRIBUTE: &[u8] = b"AXChildren\0";
    static K_AX_ROLE_ATTRIBUTE: &[u8] = b"AXRole\0";
    static K_AX_TITLE_ATTRIBUTE: &[u8] = b"AXTitle\0";
    static K_AX_POSITION_ATTRIBUTE: &[u8] = b"AXPosition\0";
    static K_AX_SIZE_ATTRIBUTE: &[u8] = b"AXSize\0";
    static K_AX_ENABLED_ATTRIBUTE: &[u8] = b"AXEnabled\0";
    static K_AX_TRUSTED_CHECK_OPTION_PROMPT: &[u8] = b"AXTrustedCheckOptionPrompt\0";

    // Clickable roles - focused on truly interactive elements
    static CLICKABLE_ROLES: &[&str] = &[
        "AXButton",             // 按钮
        "AXLink",               // 链接
        "AXMenuItem",           // 菜单项
        "AXMenuBarItem",        // 菜单栏项
        "AXPopUpButton",        // 下拉按钮
        "AXCheckBox",           // 复选框
        "AXRadioButton",        // 单选按钮
        "AXTab",                // 标签页
        "AXTabGroup",           // 标签组
        "AXDisclosureTriangle", // 展开/折叠三角
        "AXComboBox",           // 组合框
        "AXTextField",          // 文本输入框
        "AXTextArea",           // 文本区域
        "AXSlider",             // 滑块
        "AXIncrementor",        // 增减器
        "AXColorWell",          // 颜色选择器
        "AXSegmentedControl",   // 分段控件
        "AXToolbarButton",      // 工具栏按钮
        "AXSwitch",             // 开关
    ];

    fn create_cf_string(s: &[u8]) -> CFStringRef {
        unsafe {
            CFStringCreateWithCString(
                ptr::null(),
                s.as_ptr() as *const i8,
                K_CF_STRING_ENCODING_UTF8,
            )
        }
    }

    fn cf_string_to_rust(cf_string: CFStringRef) -> Option<String> {
        if cf_string.is_null() {
            return None;
        }
        unsafe {
            let mut buffer = [0i8; 256];
            if CFStringGetCString(cf_string, buffer.as_mut_ptr(), 256, K_CF_STRING_ENCODING_UTF8)
                != NO
            {
                let cstr = std::ffi::CStr::from_ptr(buffer.as_ptr());
                cstr.to_str().ok().map(|s| s.to_string())
            } else {
                None
            }
        }
    }

    fn get_ax_attribute_string(element: AXUIElementRef, attr: &[u8]) -> Option<String> {
        unsafe {
            let attr_name = create_cf_string(attr);
            let mut value: CFTypeRef = ptr::null();

            let err = AXUIElementCopyAttributeValue(element, attr_name, &mut value);
            CFRelease(attr_name as CFTypeRef);

            if err != AX_ERROR_SUCCESS || value.is_null() {
                return None;
            }

            // Check if it's a string
            if CFGetTypeID(value) == CFStringGetTypeID() {
                let result = cf_string_to_rust(value as CFStringRef);
                CFRelease(value);
                result
            } else {
                CFRelease(value);
                None
            }
        }
    }

    fn get_ax_attribute_array(element: AXUIElementRef, attr: &[u8]) -> Option<Vec<AXUIElementRef>> {
        unsafe {
            let attr_name = create_cf_string(attr);
            let mut value: CFTypeRef = ptr::null();

            let err = AXUIElementCopyAttributeValue(element, attr_name, &mut value);
            CFRelease(attr_name as CFTypeRef);

            if err != AX_ERROR_SUCCESS || value.is_null() {
                return None;
            }

            let count = CFArrayGetCount(value as CFArrayRef);
            let mut elements = Vec::with_capacity(count as usize);

            for i in 0..count {
                let item = CFArrayGetValueAtIndex(value as CFArrayRef, i);
                if !item.is_null() {
                    elements.push(item as AXUIElementRef);
                }
            }

            // Note: We don't release the array items as they're owned by the array
            // The caller should handle the array lifetime
            Some(elements)
        }
    }

    fn get_ax_position(element: AXUIElementRef) -> Option<CGPoint> {
        unsafe {
            let attr_name = create_cf_string(K_AX_POSITION_ATTRIBUTE);
            let mut value: CFTypeRef = ptr::null();

            let err = AXUIElementCopyAttributeValue(element, attr_name, &mut value);
            CFRelease(attr_name as CFTypeRef);

            if err != AX_ERROR_SUCCESS || value.is_null() {
                return None;
            }

            let mut point = CGPoint::default();
            let success = AXValueGetValue(
                value as AXValueRef,
                AX_VALUE_TYPE_CGPOINT,
                &mut point as *mut _ as *mut c_void,
            );
            CFRelease(value);

            if success { Some(point) } else { None }
        }
    }

    fn get_ax_size(element: AXUIElementRef) -> Option<CGSize> {
        unsafe {
            let attr_name = create_cf_string(K_AX_SIZE_ATTRIBUTE);
            let mut value: CFTypeRef = ptr::null();

            let err = AXUIElementCopyAttributeValue(element, attr_name, &mut value);
            CFRelease(attr_name as CFTypeRef);

            if err != AX_ERROR_SUCCESS || value.is_null() {
                return None;
            }

            let mut size = CGSize::default();
            let success = AXValueGetValue(
                value as AXValueRef,
                AX_VALUE_TYPE_CGSIZE,
                &mut size as *mut _ as *mut c_void,
            );
            CFRelease(value);

            if success { Some(size) } else { None }
        }
    }

    fn is_element_enabled(element: AXUIElementRef) -> bool {
        unsafe {
            let attr_name = create_cf_string(K_AX_ENABLED_ATTRIBUTE);
            let mut value: CFTypeRef = ptr::null();

            let err = AXUIElementCopyAttributeValue(element, attr_name, &mut value);
            CFRelease(attr_name as CFTypeRef);

            if err != AX_ERROR_SUCCESS || value.is_null() {
                return true; // Assume enabled if we can't check
            }

            let result = CFBooleanGetValue(value) != NO;
            CFRelease(value);
            result
        }
    }

    fn is_clickable_role(role: &str) -> bool {
        CLICKABLE_ROLES.contains(&role)
    }

    /// macOS Accessibility Provider using AXUIElement API
    pub struct MacOSAccessibility;

    impl MacOSAccessibility {
        pub fn new() -> Self {
            Self
        }

        /// Maximum elements to collect (prevents hangs on complex UIs)
        const MAX_ELEMENTS: usize = 300;
        /// Maximum depth for traversal
        const MAX_DEPTH: usize = 20;

        #[allow(clippy::only_used_in_recursion)]
        fn traverse_element(
            &self,
            element: AXUIElementRef,
            elements: &mut Vec<ClickableElement>,
            depth: usize,
        ) {
            // Limit recursion depth and element count
            if depth > Self::MAX_DEPTH || element.is_null() || elements.len() >= Self::MAX_ELEMENTS
            {
                return;
            }

            // Get element role
            let role = match get_ax_attribute_string(element, K_AX_ROLE_ATTRIBUTE) {
                Some(r) => r,
                None => return,
            };

            // Check for web content skip early (before potentially moving role)
            let skip_web_content = role == "AXWebArea" && depth > 8;

            // Check if this element is clickable
            if is_clickable_role(&role) {
                // Check if element is enabled
                if is_element_enabled(element) {
                    // Get position and size
                    if let (Some(pos), Some(size)) =
                        (get_ax_position(element), get_ax_size(element))
                    {
                        // Skip elements with zero/negative size or very small elements
                        if size.width > 10.0 && size.height > 10.0 {
                            // Get title/name for display
                            let name = get_ax_attribute_string(element, K_AX_TITLE_ATTRIBUTE)
                                .or_else(|| get_ax_attribute_string(element, b"AXDescription\0"))
                                .or_else(|| get_ax_attribute_string(element, b"AXHelp\0"));

                            let bounds =
                                (pos.x as f32, pos.y as f32, size.width as f32, size.height as f32);

                            elements.push(ClickableElement {
                                position: (bounds.0 + bounds.2 / 2.0, bounds.1 + bounds.3 / 2.0),
                                bounds,
                                role,
                                name,
                            });
                        }
                    }
                }
            }

            // Always recurse into children to find nested buttons/links

            if !skip_web_content {
                if let Some(children) = get_ax_attribute_array(element, K_AX_CHILDREN_ATTRIBUTE) {
                    for child in children {
                        if elements.len() >= Self::MAX_ELEMENTS {
                            break;
                        }
                        self.traverse_element(child, elements, depth + 1);
                    }
                }
            }
        }

        #[allow(dead_code)]
        fn get_frontmost_app_pid() -> Option<i32> {
            unsafe {
                let workspace_class = Class::get("NSWorkspace")?;
                let workspace: *mut Object = msg_send![workspace_class, sharedWorkspace];
                if workspace.is_null() {
                    return None;
                }

                let frontmost_app: *mut Object = msg_send![workspace, frontmostApplication];
                if frontmost_app.is_null() {
                    return None;
                }

                let pid: i32 = msg_send![frontmost_app, processIdentifier];
                Some(pid)
            }
        }

        /// Get PIDs of all running applications
        #[allow(dead_code)]
        fn get_all_app_pids() -> Vec<i32> {
            unsafe {
                let workspace_class = match Class::get("NSWorkspace") {
                    Some(c) => c,
                    None => return vec![],
                };
                let workspace: *mut Object = msg_send![workspace_class, sharedWorkspace];
                if workspace.is_null() {
                    return vec![];
                }

                let running_apps: *mut Object = msg_send![workspace, runningApplications];
                if running_apps.is_null() {
                    return vec![];
                }

                let count: usize = msg_send![running_apps, count];
                let mut pids = Vec::with_capacity(count);

                for i in 0..count {
                    let app: *mut Object = msg_send![running_apps, objectAtIndex: i];
                    if !app.is_null() {
                        // Check if app is active (has windows on screen)
                        let activation_policy: i64 = msg_send![app, activationPolicy];
                        // 0 = regular app, 1 = accessory, 2 = prohibited
                        if activation_policy == 0 {
                            let pid: i32 = msg_send![app, processIdentifier];
                            pids.push(pid);
                        }
                    }
                }

                pids
            }
        }

        /// Get the frontmost application PID, excluding the specified PID
        /// If the frontmost app is the excluded one, get the next most recent app
        fn get_frontmost_app_pid_excluding(exclude_pid: i32) -> Option<i32> {
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
    }

    impl Default for MacOSAccessibility {
        fn default() -> Self {
            Self::new()
        }
    }

    impl AccessibilityProvider for MacOSAccessibility {
        fn get_clickable_elements(&self) -> Result<Vec<ClickableElement>> {
            if !self.is_trusted() {
                return Err(AumateError::Other(
                    "Accessibility permission not granted. Please enable it in System Settings > Privacy & Security > Accessibility".to_string()
                ));
            }

            let mut elements = Vec::new();

            // Get our own PID to skip ourselves
            let our_pid = std::process::id() as i32;

            // Get the frontmost application (excluding ourselves)
            let pid = Self::get_frontmost_app_pid_excluding(our_pid)
                .ok_or_else(|| AumateError::Other("No frontmost application found".to_string()))?;

            unsafe {
                let app = AXUIElementCreateApplication(pid);
                if app.is_null() {
                    return Err(AumateError::Other(
                        "Failed to create AXUIElement for application".to_string(),
                    ));
                }

                // Get windows
                if let Some(windows) = get_ax_attribute_array(app, K_AX_WINDOWS_ATTRIBUTE) {
                    for window in windows {
                        self.traverse_element(window, &mut elements, 0);
                    }
                }

                CFRelease(app as CFTypeRef);
            }

            log::debug!("Found {} clickable elements", elements.len());
            Ok(elements)
        }

        fn is_trusted(&self) -> bool {
            unsafe { AXIsProcessTrusted() != NO }
        }

        fn request_permission(&self) {
            unsafe {
                let key = create_cf_string(K_AX_TRUSTED_CHECK_OPTION_PROMPT);

                // Store kCFBooleanTrue in a local variable to get a stable address
                let key_ref: CFTypeRef = key as CFTypeRef;
                let value_ref: CFTypeRef = kCFBooleanTrue;

                let dict = CFDictionaryCreate(
                    ptr::null(),
                    &key_ref,
                    &value_ref,
                    1,
                    ptr::null(),
                    ptr::null(),
                );

                AXIsProcessTrustedWithOptions(dict);
                CFRelease(dict);
                CFRelease(key as CFTypeRef);
            }
        }
    }
}

#[cfg(target_os = "macos")]
pub use macos::MacOSAccessibility;

/// Create the platform-specific accessibility provider
#[cfg(target_os = "macos")]
pub fn create_provider() -> Box<dyn AccessibilityProvider> {
    Box::new(MacOSAccessibility::new())
}

#[cfg(not(target_os = "macos"))]
pub fn create_provider() -> Box<dyn AccessibilityProvider> {
    Box::new(StubAccessibility)
}

/// Stub implementation for non-macOS platforms
#[cfg(not(target_os = "macos"))]
pub struct StubAccessibility;

#[cfg(not(target_os = "macos"))]
impl AccessibilityProvider for StubAccessibility {
    fn get_clickable_elements(&self) -> Result<Vec<ClickableElement>> {
        Err(AumateError::Other("Click Helper is only supported on macOS".to_string()))
    }

    fn is_trusted(&self) -> bool {
        false
    }

    fn request_permission(&self) {}
}
