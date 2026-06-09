pub mod event;

use std::collections::HashMap;

use crate::{platform::linux::x11::xwindow::X11Window, window::event::Event};

pub type KeyStates = HashMap<u32, bool>;
pub type ButtonStates = HashMap<u32, bool>;

pub struct Window {
    xwindow: X11Window,
    key_states: KeyStates,
    pub button_states: ButtonStates,
}

impl Window {
    pub fn new() -> Window {
        Self {
            xwindow: X11Window::new(),
            key_states: KeyStates::new(),
            button_states: ButtonStates::new(),
        }
    }

    pub fn next_event(&mut self) -> Event {
        Event::decode_event(&self.xwindow, &mut self.key_states, &mut self.button_states)
    }
}
