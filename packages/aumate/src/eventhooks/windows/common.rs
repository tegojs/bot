//! Common utilities for Windows event handling

use crate::eventhooks::keycodes::windows::key_from_code;
use crate::eventhooks::types::{Button, EventType};
use std::convert::TryInto;
use std::os::raw::c_short;
use winapi::shared::minwindef::{DWORD, HIWORD, LPARAM, WORD, WPARAM};
use winapi::shared::ntdef::LONG;
use winapi::um::winuser::{
    KBDLLHOOKSTRUCT, MSLLHOOKSTRUCT, VK_PACKET, WHEEL_DELTA, WM_KEYDOWN, WM_KEYUP, WM_LBUTTONDOWN,
    WM_LBUTTONUP, WM_MBUTTONDOWN, WM_MBUTTONUP, WM_MOUSEHWHEEL, WM_MOUSEMOVE, WM_MOUSEWHEEL,
    WM_RBUTTONDOWN, WM_RBUTTONUP, WM_SYSKEYDOWN, WM_SYSKEYUP, WM_XBUTTONDOWN, WM_XBUTTONUP,
};

/// Get virtual key code from low-level keyboard hook data
pub unsafe fn get_code(lpdata: LPARAM) -> DWORD {
    let kb = *(lpdata as *const KBDLLHOOKSTRUCT);
    if kb.vkCode == VK_PACKET as u32 { kb.scanCode } else { kb.vkCode }
}

/// Get scan code from low-level keyboard hook data
pub unsafe fn get_scan_code(lpdata: LPARAM) -> DWORD {
    let kb = *(lpdata as *const KBDLLHOOKSTRUCT);
    match kb.scanCode {
        0x36 | 0x45 => kb.scanCode,
        _ => {
            if (kb.flags & 0x01) == 0x01 {
                0xE0 << 8 | kb.scanCode
            } else {
                kb.scanCode
            }
        }
    }
}

/// Get mouse position from low-level mouse hook data
pub unsafe fn get_point(lpdata: LPARAM) -> (LONG, LONG) {
    let mouse = *(lpdata as *const MSLLHOOKSTRUCT);
    (mouse.pt.x, mouse.pt.y)
}

/// Get mouse wheel delta
pub unsafe fn get_delta(lpdata: LPARAM) -> WORD {
    let mouse = *(lpdata as *const MSLLHOOKSTRUCT);
    HIWORD(mouse.mouseData)
}

/// Get mouse button code for X buttons
pub unsafe fn get_button_code(lpdata: LPARAM) -> WORD {
    let mouse = *(lpdata as *const MSLLHOOKSTRUCT);
    HIWORD(mouse.mouseData)
}

/// Convert Windows hook message to EventType
pub unsafe fn convert(param: WPARAM, lpdata: LPARAM) -> (Option<EventType>, u16) {
    let mut code = 0u16;

    let event_type = match param.try_into() {
        Ok(WM_KEYDOWN) | Ok(WM_SYSKEYDOWN) => {
            code = get_code(lpdata) as u16;
            let key = key_from_code(code as u32);
            Some(EventType::KeyPress(key))
        }
        Ok(WM_KEYUP) | Ok(WM_SYSKEYUP) => {
            code = get_code(lpdata) as u16;
            let key = key_from_code(code as u32);
            Some(EventType::KeyRelease(key))
        }
        Ok(WM_LBUTTONDOWN) => Some(EventType::ButtonPress(Button::Left)),
        Ok(WM_LBUTTONUP) => Some(EventType::ButtonRelease(Button::Left)),
        Ok(WM_MBUTTONDOWN) => Some(EventType::ButtonPress(Button::Middle)),
        Ok(WM_MBUTTONUP) => Some(EventType::ButtonRelease(Button::Middle)),
        Ok(WM_RBUTTONDOWN) => Some(EventType::ButtonPress(Button::Right)),
        Ok(WM_RBUTTONUP) => Some(EventType::ButtonRelease(Button::Right)),
        Ok(WM_XBUTTONDOWN) => {
            let btn_code = get_button_code(lpdata) as u8;
            Some(EventType::ButtonPress(Button::Unknown(btn_code)))
        }
        Ok(WM_XBUTTONUP) => {
            let btn_code = get_button_code(lpdata) as u8;
            Some(EventType::ButtonRelease(Button::Unknown(btn_code)))
        }
        Ok(WM_MOUSEMOVE) => {
            let (x, y) = get_point(lpdata);
            Some(EventType::MouseMove { x: x as f64, y: y as f64 })
        }
        Ok(WM_MOUSEWHEEL) => {
            let delta = get_delta(lpdata) as c_short;
            Some(EventType::Wheel { delta_x: 0, delta_y: (delta / WHEEL_DELTA) as i64 })
        }
        Ok(WM_MOUSEHWHEEL) => {
            let delta = get_delta(lpdata) as c_short;
            Some(EventType::Wheel { delta_x: (delta / WHEEL_DELTA) as i64, delta_y: 0 })
        }
        _ => None,
    };

    (event_type, code)
}
