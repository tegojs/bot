//! Windows global event grab using low-level hooks

use crate::eventhooks::types::{Event, GrabError};
use crate::eventhooks::windows::common::{convert, get_scan_code};
use std::io::Error;
use std::ptr::null_mut;
use std::sync::Mutex;
use std::time::SystemTime;
use winapi::shared::basetsd::ULONG_PTR;
use winapi::shared::minwindef::{DWORD, FALSE, LPARAM, LRESULT, WPARAM};
use winapi::shared::ntdef::NULL;
use winapi::shared::windef::{HHOOK, POINT};
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::processthreadsapi::GetCurrentThreadId;
use winapi::um::winuser::{
    CallNextHookEx, DispatchMessageA, GetMessageA, HC_ACTION, MSG, PKBDLLHOOKSTRUCT,
    PMOUSEHOOKSTRUCT, PostThreadMessageA, SetWindowsHookExA, TranslateMessage, UnhookWindowsHookEx,
    WH_KEYBOARD_LL, WH_MOUSE_LL, WM_USER,
};

static mut GLOBAL_CALLBACK: Option<Box<dyn FnMut(Event) -> Option<Event>>> = None;

lazy_static::lazy_static! {
    static ref CUR_HOOK_THREAD_ID: Mutex<DWORD> = Mutex::new(0);
}

const WM_USER_EXIT_HOOK: u32 = WM_USER + 1;

/// Raw callback for processing hook events
unsafe fn raw_callback(
    code: i32,
    param: WPARAM,
    lpdata: LPARAM,
    f_get_extra_data: impl FnOnce(LPARAM) -> ULONG_PTR,
) -> LRESULT {
    if code == HC_ACTION as i32 {
        let (opt, vk_code) = convert(param, lpdata);
        if let Some(event_type) = opt {
            let event =
                Event { event_type, time: SystemTime::now(), platform_code: get_scan_code(lpdata) };
            // Suppress unused warning - extra_data could be used for filtering
            let _ = f_get_extra_data(lpdata);

            if let Some(callback) = &mut GLOBAL_CALLBACK {
                if callback(event).is_none() {
                    // Block the event
                    return 1;
                }
            }
        }
    }
    CallNextHookEx(null_mut(), code, param, lpdata)
}

/// Mouse hook callback
unsafe extern "system" fn raw_callback_mouse(code: i32, param: WPARAM, lpdata: LPARAM) -> LRESULT {
    raw_callback(code, param, lpdata, |data: LPARAM| (*(data as PMOUSEHOOKSTRUCT)).dwExtraInfo)
}

/// Keyboard hook callback
unsafe extern "system" fn raw_callback_keyboard(
    code: i32,
    param: WPARAM,
    lpdata: LPARAM,
) -> LRESULT {
    raw_callback(code, param, lpdata, |data: LPARAM| (*(data as PKBDLLHOOKSTRUCT)).dwExtraInfo)
}

/// Install hooks
fn do_hook<T>(callback: T) -> Result<(HHOOK, HHOOK), GrabError>
where
    T: FnMut(Event) -> Option<Event> + 'static,
{
    let mut cur_hook_thread_id = CUR_HOOK_THREAD_ID.lock().unwrap();
    if *cur_hook_thread_id != 0 {
        return Ok((null_mut(), null_mut()));
    }

    unsafe {
        GLOBAL_CALLBACK = Some(Box::new(callback));

        let hook_keyboard =
            SetWindowsHookExA(WH_KEYBOARD_LL, Some(raw_callback_keyboard), null_mut(), 0);
        if hook_keyboard.is_null() {
            return Err(GrabError::KeyHookError(GetLastError()));
        }

        let hook_mouse = SetWindowsHookExA(WH_MOUSE_LL, Some(raw_callback_mouse), null_mut(), 0);
        if hook_mouse.is_null() {
            if FALSE == UnhookWindowsHookEx(hook_keyboard) {
                log::error!("UnhookWindowsHookEx keyboard: {}", Error::last_os_error());
            }
            return Err(GrabError::MouseHookError(GetLastError()));
        }

        *cur_hook_thread_id = GetCurrentThreadId();
        Ok((hook_keyboard, hook_mouse))
    }
}

/// Check if grab is active
#[inline]
pub fn is_grabbed() -> bool {
    *CUR_HOOK_THREAD_ID.lock().unwrap() != 0
}

/// Start grabbing global events (blocks until exit_grab is called)
pub fn grab<T>(callback: T) -> Result<(), GrabError>
where
    T: FnMut(Event) -> Option<Event> + 'static,
{
    if is_grabbed() {
        return Ok(());
    }

    unsafe {
        let (mut hook_keyboard, hook_mouse) = do_hook(callback)?;
        if hook_keyboard.is_null() && hook_mouse.is_null() {
            return Ok(());
        }

        let mut msg = MSG {
            hwnd: NULL as _,
            message: 0,
            wParam: 0,
            lParam: 0,
            time: 0,
            pt: POINT { x: 0, y: 0 },
        };

        while FALSE != GetMessageA(&mut msg, NULL as _, 0, 0) {
            if msg.message == WM_USER_EXIT_HOOK {
                if !hook_keyboard.is_null() {
                    if FALSE == UnhookWindowsHookEx(hook_keyboard) {
                        log::error!(
                            "Failed UnhookWindowsHookEx keyboard: {}",
                            Error::last_os_error()
                        );
                        continue;
                    }
                    hook_keyboard = null_mut();
                }

                if !hook_mouse.is_null() {
                    if FALSE == UnhookWindowsHookEx(hook_mouse) {
                        log::error!("Failed UnhookWindowsHookEx mouse: {}", Error::last_os_error());
                        continue;
                    }
                }
                break;
            }

            TranslateMessage(&msg);
            DispatchMessageA(&msg);
        }

        *CUR_HOOK_THREAD_ID.lock().unwrap() = 0;
    }
    Ok(())
}

/// Stop grabbing events
pub fn exit_grab() -> Result<(), GrabError> {
    unsafe {
        let mut cur_hook_thread_id = CUR_HOOK_THREAD_ID.lock().unwrap();
        if *cur_hook_thread_id != 0 {
            if FALSE == PostThreadMessageA(*cur_hook_thread_id, WM_USER_EXIT_HOOK, 0, 0) {
                return Err(GrabError::ExitGrabError(format!(
                    "Failed to post exit message: {}",
                    GetLastError()
                )));
            }
        }
        *cur_hook_thread_id = 0;
    }
    Ok(())
}
