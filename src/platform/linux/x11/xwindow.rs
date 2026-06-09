use std::ffi::CString;

use super::atoms::Atoms;
use crate::platform::linux::x11::{event_mask, raw, xevent};

pub struct X11Window {
    pub display: *mut raw::Display,
    _screen_number: i32,
    _window: raw::Window,
    _parent_window: raw::Window,
    atoms: Atoms,
}

impl X11Window {
    fn setup_atoms(&mut self) {
        unsafe {
            self.atoms.wm_delete_window = raw::XInternAtom(
                self.display,
                CString::new("WM_DELETE_WINDOW").unwrap().as_ptr(),
                0,
            );
        }
    }

    pub fn get_atoms(&self) -> Atoms {
        self.atoms
    }

    pub fn new() -> Self {
        unsafe {
            let display = raw::XOpenDisplay(std::ptr::null());

            assert!(!display.is_null());

            let screen_number = raw::XDefaultScreen(display);

            let parent_window = raw::XRootWindow(display, screen_number);
            let window = raw::XCreateSimpleWindow(display, parent_window, 0, 0, 800, 600, 1, 0, 0);
            raw::XSelectInput(
                display,
                window,
                (event_mask::EventMask::exposure()
                    | event_mask::EventMask::key_press()
                    | event_mask::EventMask::structure_notify())
                .into(),
            );
            raw::XMapWindow(display, window);

            let mut xwindow = Self {
                display,
                _screen_number: screen_number,
                _parent_window: parent_window,
                _window: window,
                atoms: Atoms::default(),
            };
            xwindow.setup_atoms();
            xwindow
        }
    }

    pub fn next_event(&self) -> xevent::XEvent {
        let mut event = xevent::XEvent { data: [0u8; 192] };
        unsafe {
            raw::XNextEvent(self.display, &mut event);
        };
        event
    }
}

impl Drop for X11Window {
    fn drop(&mut self) {
        unsafe {
            raw::XCloseDisplay(self.display);
        }
    }
}
