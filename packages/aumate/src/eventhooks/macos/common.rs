//! Common types and FFI declarations for macOS event handling
#![allow(clippy::upper_case_acronyms)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes_definitions)]

use crate::eventhooks::keycodes::macos::key_from_code;
use crate::eventhooks::types::{Button, Event, EventType};
use cocoa::base::id;
use core_graphics::event::{
    CGEvent, CGEventFlags, CGEventTapLocation, CGEventType, CGKeyCode, EventField,
};
use std::convert::TryInto;
use std::os::raw::c_void;
use std::time::SystemTime;

// Type aliases for FFI
pub type CFMachPortRef = *const c_void;
pub type CFIndex = u64;
pub type CFAllocatorRef = id;
pub type CFRunLoopSourceRef = id;
pub type CFRunLoopRef = id;
pub type CFRunLoopMode = id;
pub type CGEventTapProxy = id;
pub type CGEventRef = CGEvent;

// CGEventTap placement
pub type CGEventTapPlacement = u32;
pub const kCGHeadInsertEventTap: u32 = 0;

// CGEventTap options
#[repr(u32)]
pub enum CGEventTapOption {
    Default = 0,
    #[allow(dead_code)]
    ListenOnly = 1,
}

// Event mask for all events we care about
pub type CGEventMask = u64;
pub const kCGEventMaskForAllEvents: u64 = (1 << CGEventType::LeftMouseDown as u64)
    + (1 << CGEventType::LeftMouseUp as u64)
    + (1 << CGEventType::RightMouseDown as u64)
    + (1 << CGEventType::RightMouseUp as u64)
    + (1 << CGEventType::OtherMouseDown as u64)
    + (1 << CGEventType::OtherMouseUp as u64)
    + (1 << CGEventType::MouseMoved as u64)
    + (1 << CGEventType::LeftMouseDragged as u64)
    + (1 << CGEventType::RightMouseDragged as u64)
    + (1 << CGEventType::KeyDown as u64)
    + (1 << CGEventType::KeyUp as u64)
    + (1 << CGEventType::FlagsChanged as u64)
    + (1 << CGEventType::ScrollWheel as u64);

// Track last flags for modifier key detection
pub static mut LAST_FLAGS: CGEventFlags = CGEventFlags::CGEventFlagNull;

// FFI declarations
#[cfg(target_os = "macos")]
#[link(name = "Cocoa", kind = "framework")]
unsafe extern "C" {
    #[allow(improper_ctypes)]
    pub fn CGEventTapCreate(
        tap: CGEventTapLocation,
        place: CGEventTapPlacement,
        options: CGEventTapOption,
        eventsOfInterest: CGEventMask,
        callback: QCallback,
        user_info: id,
    ) -> CFMachPortRef;

    pub fn CFMachPortCreateRunLoopSource(
        allocator: CFAllocatorRef,
        tap: CFMachPortRef,
        order: CFIndex,
    ) -> CFRunLoopSourceRef;

    pub fn CFRunLoopGetCurrent() -> CFRunLoopRef;
    pub fn CFRunLoopAddSource(rl: CFRunLoopRef, source: CFRunLoopSourceRef, mode: CFRunLoopMode);
    pub fn CGEventTapEnable(tap: CFMachPortRef, enable: bool);
    pub fn CFRunLoopRun();
    pub fn CFRunLoopStop(rl: CFRunLoopRef);

    pub static kCFRunLoopCommonModes: CFRunLoopMode;
}

// Callback type for CGEventTapCreate
pub type QCallback = unsafe extern "C" fn(
    proxy: CGEventTapProxy,
    _type: CGEventType,
    cg_event: CGEventRef,
    user_info: *mut c_void,
) -> CGEventRef;

/// Get keycode from CGEvent
#[inline]
unsafe fn get_code(cg_event: &CGEvent) -> Option<CGKeyCode> {
    cg_event.get_integer_value_field(EventField::KEYBOARD_EVENT_KEYCODE).try_into().ok()
}

/// Convert CGEvent to our Event type
pub unsafe fn convert(_type: CGEventType, cg_event: &CGEvent) -> Option<Event> {
    let mut code: CGKeyCode = 0;

    let option_type = match _type {
        CGEventType::LeftMouseDown => Some(EventType::ButtonPress(Button::Left)),
        CGEventType::LeftMouseUp => Some(EventType::ButtonRelease(Button::Left)),
        CGEventType::RightMouseDown => Some(EventType::ButtonPress(Button::Right)),
        CGEventType::RightMouseUp => Some(EventType::ButtonRelease(Button::Right)),
        CGEventType::OtherMouseDown => {
            match cg_event.get_integer_value_field(EventField::MOUSE_EVENT_BUTTON_NUMBER) {
                2 => Some(EventType::ButtonPress(Button::Middle)),
                n => Some(EventType::ButtonPress(Button::Unknown(n as u8))),
            }
        }
        CGEventType::OtherMouseUp => {
            match cg_event.get_integer_value_field(EventField::MOUSE_EVENT_BUTTON_NUMBER) {
                2 => Some(EventType::ButtonRelease(Button::Middle)),
                n => Some(EventType::ButtonRelease(Button::Unknown(n as u8))),
            }
        }
        CGEventType::MouseMoved
        | CGEventType::LeftMouseDragged
        | CGEventType::RightMouseDragged => {
            let point = cg_event.location();
            Some(EventType::MouseMove { x: point.x, y: point.y })
        }
        CGEventType::KeyDown => {
            code = unsafe { get_code(cg_event)? };
            Some(EventType::KeyPress(key_from_code(code)))
        }
        CGEventType::KeyUp => {
            code = unsafe { get_code(cg_event)? };
            Some(EventType::KeyRelease(key_from_code(code)))
        }
        CGEventType::FlagsChanged => {
            code = unsafe { get_code(cg_event)? };
            let flags = cg_event.get_flags();
            // SAFETY: We're the only thread accessing this static in this context
            if flags < unsafe { LAST_FLAGS } {
                unsafe { LAST_FLAGS = flags };
                Some(EventType::KeyRelease(key_from_code(code)))
            } else {
                unsafe { LAST_FLAGS = flags };
                Some(EventType::KeyPress(key_from_code(code)))
            }
        }
        CGEventType::ScrollWheel => {
            let delta_y =
                cg_event.get_integer_value_field(EventField::SCROLL_WHEEL_EVENT_POINT_DELTA_AXIS_1);
            let delta_x =
                cg_event.get_integer_value_field(EventField::SCROLL_WHEEL_EVENT_POINT_DELTA_AXIS_2);
            Some(EventType::Wheel { delta_x, delta_y })
        }
        _ => None,
    };

    option_type.map(|event_type| Event {
        time: SystemTime::now(),
        event_type,
        platform_code: code as u32,
    })
}
