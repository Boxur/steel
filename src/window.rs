pub mod event;

use crate::{platform::linux::x11::xwindow::X11Window, window::event::Event};
pub struct Window {
    xwindow: X11Window,
}

impl Window {
    pub fn new() -> Window {
        Self {
            xwindow: X11Window::new(),
        }
    }

    pub fn next_event(&self) -> Event {
        Event::decode_event(&self.xwindow)
    }
}
