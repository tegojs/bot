//! Common utilities for Linux event handling

use crate::eventhooks::keycodes::linux::key_from_code;
use crate::eventhooks::types::{Event, EventType};
use std::time::SystemTime;

const KEYPRESS_EVENT: i32 = 2;

/// Convert X11 key event to our Event type
pub fn convert_event(code: u32, event_type: i32) -> Event {
    let key = key_from_code(code);
    let event_type = if event_type == KEYPRESS_EVENT {
        EventType::KeyPress(key)
    } else {
        EventType::KeyRelease(key)
    };

    Event { time: SystemTime::now(), event_type, platform_code: code }
}
