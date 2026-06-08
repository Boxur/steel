mod event_mask;
pub mod raw;
use crate::platform::linux::x11::raw::*;
use event_mask::EventMask;

pub struct X11Window {
    pub display: *mut Display,
    screen_number: i32,
    window: Window,
    parent_window: Window,
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
                screen_number,
                parent_window,
                window,
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
