pub mod event;
mod event_mask;
pub mod raw;
use crate::platform::linux::x11::{event::XEvent, raw::*};
use event_mask::EventMask;

pub struct X11Window {
    pub display: *mut Display,
    _screen_number: i32,
    _window: Window,
    _parent_window: Window,
}

impl X11Window {
    pub fn new() -> Self {
        unsafe {
            let display = XOpenDisplay(std::ptr::null());

            assert!(!display.is_null());

            let screen_number = XDefaultScreen(display);

            let parent_window = XRootWindow(display, screen_number);
            let window = XCreateSimpleWindow(display, parent_window, 0, 0, 800, 600, 1, 0, 0);
            XSelectInput(
                display,
                window,
                (EventMask::exposure() | EventMask::key_press() | EventMask::structure_notify())
                    .into(),
            );
            XMapWindow(display, window);

            Self {
                display,
                _screen_number: screen_number,
                _parent_window: parent_window,
                _window: window,
            }
        }
    }

    pub fn next_event(&self) -> XEvent {
        let mut event = XEvent { data: [0u8; 192] };
        unsafe {
            XNextEvent(self.display, &mut event);
        };
        event
    }
}

impl Drop for X11Window {
    fn drop(&mut self) {
        unsafe {
            XCloseDisplay(self.display);
        }
    }
}
