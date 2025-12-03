//! macOS global event grab using CGEventTap
#![allow(improper_ctypes_definitions)]

use crate::eventhooks::macos::common::*;
use crate::eventhooks::types::{Event, GrabError};
use cocoa::base::nil;
use cocoa::foundation::NSAutoreleasePool;
use core_graphics::event::{CGEventTapLocation, CGEventType};
use std::os::raw::c_void;
use std::sync::Mutex;

/// Callback type for grab events
type GrabCallbackBox = Box<dyn FnMut(Event) -> Option<Event> + Send>;

// Global callback storage using Mutex for safe access
static GLOBAL_CALLBACK: Mutex<Option<GrabCallbackBox>> = Mutex::new(None);

// Current run loop for stopping
static mut CUR_LOOP: CFRunLoopSourceRef = std::ptr::null_mut();

/// Raw callback invoked by CGEventTap
unsafe extern "C" fn raw_callback(
    _proxy: CGEventTapProxy,
    _type: CGEventType,
    cg_event: CGEventRef,
    _user_info: *mut c_void,
) -> CGEventRef {
    // SAFETY: convert is unsafe, we're in an unsafe extern fn context
    if let Some(event) = unsafe { convert(_type, &cg_event) } {
        // Access callback through mutex
        if let Ok(mut guard) = GLOBAL_CALLBACK.lock() {
            if let Some(callback) = guard.as_mut() {
                if callback(event).is_none() {
                    // Consume the event by setting type to Null
                    cg_event.set_type(CGEventType::Null);
                }
            }
        }
    }
    cg_event
}

/// Check if grab is currently active
#[inline]
pub fn is_grabbed() -> bool {
    unsafe { !CUR_LOOP.is_null() }
}

/// Start grabbing global events
///
/// This function blocks until `exit_grab()` is called.
/// The callback receives events and returns:
/// - `Some(event)` to let the event pass through
/// - `None` to consume/block the event
pub fn grab<T>(callback: T) -> Result<(), GrabError>
where
    T: FnMut(Event) -> Option<Event> + Send + 'static,
{
    if is_grabbed() {
        return Ok(());
    }

    // Store callback in mutex
    if let Ok(mut guard) = GLOBAL_CALLBACK.lock() {
        *guard = Some(Box::new(callback));
    }

    unsafe {
        let _pool = NSAutoreleasePool::new(nil);

        // Create event tap
        let tap = CGEventTapCreate(
            CGEventTapLocation::Session,
            kCGHeadInsertEventTap,
            CGEventTapOption::Default,
            kCGEventMaskForAllEvents,
            raw_callback,
            nil,
        );

        if tap.is_null() {
            return Err(GrabError::EventTapError);
        }

        // Create run loop source
        let loop_source = CFMachPortCreateRunLoopSource(nil, tap, 0);
        if loop_source.is_null() {
            return Err(GrabError::LoopSourceError);
        }

        // Add to current run loop
        CUR_LOOP = CFRunLoopGetCurrent() as _;
        CFRunLoopAddSource(CUR_LOOP, loop_source, kCFRunLoopCommonModes);

        // Enable tap and run
        CGEventTapEnable(tap, true);
        CFRunLoopRun();
    }

    Ok(())
}

/// Stop grabbing events
pub fn exit_grab() -> Result<(), GrabError> {
    unsafe {
        if !CUR_LOOP.is_null() {
            CFRunLoopStop(CUR_LOOP);
            CUR_LOOP = std::ptr::null_mut();
        }
    }
    Ok(())
}
