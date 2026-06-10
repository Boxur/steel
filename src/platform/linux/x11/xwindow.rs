use std::ffi::CString;

use super::xatoms::XAtoms;
use crate::platform::linux::x11::{event_mask, raw, xevent};

#[derive(Debug)]
pub struct X11Window {
    pub display: *mut raw::Display,
    _screen_number: i32,
    _window: raw::Window,
    _parent_window: raw::Window,
    atoms: XAtoms,
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

    pub fn get_atoms(&self) -> &XAtoms {
        &self.atoms
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
                    | event_mask::EventMask::key_release()
                    | event_mask::EventMask::button_press()
                    | event_mask::EventMask::button_release()
                    | event_mask::EventMask::pointer_motion()
                    | event_mask::EventMask::enter_window()
                    | event_mask::EventMask::leave_window()
                    | event_mask::EventMask::focus_change()
                    | event_mask::EventMask::resize_redirect()
                    | event_mask::EventMask::structure_notify())
                .into(),
            );
            raw::XMapWindow(display, window);

            let mut xwindow = Self {
                display,
                _screen_number: screen_number,
                _parent_window: parent_window,
                _window: window,
                atoms: XAtoms::default(),
            };
            xwindow.setup_atoms();
            xwindow
        }
    }

    pub fn next_event(&self) -> xevent::Data {
        let mut data = [0u8; 192];
        unsafe {
            raw::XNextEvent(self.display, &mut data);
        };
        data
    }
}

impl Drop for X11Window {
    fn drop(&mut self) {
        unsafe {
            raw::XCloseDisplay(self.display);
        }
    }
}
