//! Linux global keyboard grab using X11 XGrabKeyboard

use crate::eventhooks::linux::common::convert_event;
use crate::eventhooks::types::{Event, GrabError};
use mio::{Events, Interest, Poll, Token, unix::SourceFd};
use std::{
    mem::zeroed,
    os::raw::c_int,
    ptr,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
        mpsc::{Receiver, Sender, channel},
    },
    thread,
    time::Duration,
};
use x11::xlib::{self, GrabModeAsync, KeyPressMask, KeyReleaseMask, Window};

// Internal event types
enum GrabEvent {
    Exit,
    KeyEvent(Event),
}

enum GrabControl {
    Grab,
    UnGrab,
    Exit,
}

// X11 keyboard grabber
struct KeyboardGrabber {
    display: *mut xlib::Display,
    screen: *mut xlib::Screen,
    window: Window,
    grab_fd: c_int,
}

unsafe impl Send for KeyboardGrabber {}
unsafe impl Sync for KeyboardGrabber {}

/// Callback type for grab events
type GrabCallbackBox = Box<dyn FnMut(Event) -> Option<Event> + Send>;

lazy_static::lazy_static! {
    static ref GRAB_KEY_EVENT_SENDER: Arc<Mutex<Option<Sender<GrabEvent>>>> = Arc::new(Mutex::new(None));
    static ref GRAB_CONTROL_SENDER: Arc<Mutex<Option<Sender<GrabControl>>>> = Arc::new(Mutex::new(None));
    static ref GLOBAL_CALLBACK: Mutex<Option<GrabCallbackBox>> = Mutex::new(None);
}

const KEYPRESS_EVENT: i32 = 2;
static IS_GRABBING: AtomicBool = AtomicBool::new(false);
const GRAB_RECV: Token = Token(0);

impl KeyboardGrabber {
    fn create() -> Result<Self, GrabError> {
        let mut grabber =
            Self { display: ptr::null_mut(), screen: ptr::null_mut(), window: 0, grab_fd: 0 };

        grabber.display = unsafe { xlib::XOpenDisplay(ptr::null()) };
        if grabber.display.is_null() {
            return Err(GrabError::MissingDisplayError);
        }

        let screen_number = unsafe { xlib::XDefaultScreen(grabber.display) };
        grabber.screen = unsafe { xlib::XScreenOfDisplay(grabber.display, screen_number) };
        if grabber.screen.is_null() {
            return Err(GrabError::MissingScreenError);
        }

        grabber.window = unsafe { xlib::XRootWindowOfScreen(grabber.screen) };
        unsafe {
            xlib::XSelectInput(grabber.display, grabber.window, KeyPressMask | KeyReleaseMask);
        }

        grabber.grab_fd = unsafe { xlib::XConnectionNumber(grabber.display) };
        Ok(grabber)
    }

    fn start(&self) -> Result<(), GrabError> {
        let poll = Poll::new().map_err(GrabError::IoError)?;
        poll.registry()
            .register(&mut SourceFd(&self.grab_fd), GRAB_RECV, Interest::READABLE)
            .map_err(GrabError::IoError)?;

        let (tx, rx) = channel();
        GRAB_CONTROL_SENDER.lock().unwrap().replace(tx);

        let display_lock = Arc::new(Mutex::new(self.display as u64));
        start_grab_control_thread(display_lock.clone(), self.window, rx);
        loop_poll_x_event(display_lock, poll);
        Ok(())
    }
}

impl Drop for KeyboardGrabber {
    fn drop(&mut self) {
        if !self.display.is_null() {
            ungrab_keys_(self.display);
            let _ = unsafe { xlib::XCloseDisplay(self.display) };
        }
    }
}

fn grab_keys(display: Arc<Mutex<u64>>, grab_window: libc::c_ulong) {
    unsafe {
        let lock = display.lock().unwrap();
        let display = *lock as *mut xlib::Display;
        xlib::XGrabKeyboard(
            display,
            grab_window,
            c_int::from(true),
            GrabModeAsync,
            GrabModeAsync,
            xlib::CurrentTime,
        );
        xlib::XFlush(display);
    }
    thread::sleep(Duration::from_millis(50));
}

fn ungrab_keys(display: Arc<Mutex<u64>>) {
    {
        let lock = display.lock().unwrap();
        let display = *lock as *mut xlib::Display;
        ungrab_keys_(display);
    }
    thread::sleep(Duration::from_millis(50));
}

fn ungrab_keys_(display: *mut xlib::Display) {
    unsafe {
        xlib::XUngrabKeyboard(display, xlib::CurrentTime);
        xlib::XFlush(display);
    }
}

fn start_callback_event_thread(recv: Receiver<GrabEvent>) {
    thread::spawn(move || {
        loop {
            if let Ok(data) = recv.recv() {
                match data {
                    GrabEvent::KeyEvent(event) => {
                        if let Ok(mut guard) = GLOBAL_CALLBACK.lock() {
                            if let Some(callback) = guard.as_mut() {
                                callback(event);
                            }
                        }
                    }
                    GrabEvent::Exit => break,
                }
            }
        }
    });
}

fn start_grab_service() -> Result<(), GrabError> {
    let (tx, rx) = channel::<GrabEvent>();
    *GRAB_KEY_EVENT_SENDER.lock().unwrap() = Some(tx);
    start_grab_thread();
    start_callback_event_thread(rx);
    Ok(())
}

fn read_x_event(x_event: &mut xlib::XEvent, display: *mut xlib::Display) {
    while (unsafe { xlib::XPending(display) }) > 0 {
        unsafe {
            xlib::XNextEvent(display, x_event);
        }
        let keycode = unsafe { x_event.key.keycode };
        let is_press = unsafe { x_event.type_ == KEYPRESS_EVENT };
        let event = convert_event(keycode, if is_press { KEYPRESS_EVENT } else { 3 });
        if let Some(tx) = GRAB_KEY_EVENT_SENDER.lock().unwrap().as_ref() {
            let _ = tx.send(GrabEvent::KeyEvent(event));
        }
    }
}

fn start_grab_control_thread(
    display: Arc<Mutex<u64>>,
    grab_window: Window,
    rx: Receiver<GrabControl>,
) {
    std::thread::spawn(move || {
        loop {
            match rx.recv() {
                Ok(evt) => match evt {
                    GrabControl::Exit => {
                        IS_GRABBING.store(false, Ordering::Relaxed);
                        break;
                    }
                    GrabControl::Grab => {
                        grab_keys(display.clone(), grab_window);
                    }
                    GrabControl::UnGrab => {
                        ungrab_keys(display.clone());
                    }
                },
                Err(e) => {
                    log::error!("Failed to receive event: {}", e);
                    break;
                }
            }
        }
    });
}

fn loop_poll_x_event(display: Arc<Mutex<u64>>, mut poll: Poll) {
    let mut x_event: xlib::XEvent = unsafe { zeroed() };
    let mut events = Events::with_capacity(128);

    loop {
        if !IS_GRABBING.load(Ordering::Relaxed) {
            break;
        }

        match poll.poll(&mut events, Some(Duration::from_millis(300))) {
            Ok(_) => {
                for event in &events {
                    if event.token() == GRAB_RECV {
                        let lock = display.lock().unwrap();
                        let display = *lock as *mut xlib::Display;
                        read_x_event(&mut x_event, display);
                    }
                }
            }
            Err(e) => {
                log::error!("Failed to poll event: {}", e);
                break;
            }
        }
    }
}

fn start_grab() -> Result<(), GrabError> {
    let grabber = KeyboardGrabber::create()?;
    grabber.start()
}

fn start_grab_thread() {
    thread::spawn(|| {
        let mut retry_count = 0;
        loop {
            if !IS_GRABBING.load(Ordering::Relaxed) {
                break;
            }
            if let Err(err) = start_grab() {
                log::debug!("Failed to start grab keyboard: {:?}", err);
                let delay = match retry_count {
                    0..=3 => {
                        retry_count += 1;
                        100
                    }
                    4..=10 => {
                        retry_count += 1;
                        retry_count * 100
                    }
                    _ => 1000,
                };
                thread::sleep(Duration::from_millis(delay as u64));
            } else {
                retry_count = 0;
            }
        }
    });
}

fn send_grab_control(data: GrabControl) {
    if let Some(sender) = GRAB_CONTROL_SENDER.lock().unwrap().as_ref() {
        if let Err(e) = sender.send(data) {
            log::error!("Failed to send grab command: {}", e);
        }
    } else {
        log::error!("Failed to send grab command: no sender");
    }
    thread::sleep(Duration::from_millis(50));
}

/// Enable keyboard grabbing
#[inline]
pub fn enable_grab() {
    send_grab_control(GrabControl::Grab);
}

/// Disable keyboard grabbing
#[inline]
pub fn disable_grab() {
    send_grab_control(GrabControl::UnGrab);
}

/// Check if grab is active
#[inline]
pub fn is_grabbed() -> bool {
    IS_GRABBING.load(Ordering::Relaxed)
}

/// Start listening for grab events
pub fn start_grab_listen<T>(callback: T) -> Result<(), GrabError>
where
    T: FnMut(Event) -> Option<Event> + Send + 'static,
{
    if is_grabbed() {
        return Ok(());
    }

    IS_GRABBING.store(true, Ordering::Relaxed);

    // Store callback in mutex
    if let Ok(mut guard) = GLOBAL_CALLBACK.lock() {
        *guard = Some(Box::new(callback));
    }

    start_grab_service()?;
    thread::sleep(Duration::from_millis(100));
    Ok(())
}

/// Stop listening for grab events
pub fn exit_grab_listen() {
    IS_GRABBING.store(false, Ordering::Relaxed);
    if let Some(tx) = GRAB_KEY_EVENT_SENDER.lock().unwrap().as_ref() {
        let _ = tx.send(GrabEvent::Exit);
    }
    send_grab_control(GrabControl::Exit);
}
