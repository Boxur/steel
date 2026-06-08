mod atoms;
pub mod event;
use std::ffi::CString;

use crate::{
    platform::linux::x11::{X11Window, raw::XInternAtom},
    window::{atoms::Atoms, event::Event},
};
pub struct Window {
    xwindow: X11Window,
    atoms: Atoms,
}

impl Window {
    fn setup_atoms(&mut self) {
        unsafe {
            self.atoms.wm_delete_window = XInternAtom(
                self.xwindow.display,
                CString::new("WM_DELETE_WINDOW").unwrap().as_ptr(),
                0,
            );
        }
    }

    pub fn new() -> Window {
        let mut window = Window {
            xwindow: X11Window::new(),
            atoms: Atoms::default(),
        };
        window.setup_atoms();
        window
    }

    pub fn next_event(&self) -> Event {
        Event::decode_event(self.xwindow.next_event(), &self.atoms)
    }
}
