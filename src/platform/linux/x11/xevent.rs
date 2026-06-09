use crate::{
    platform::linux::x11::{atoms, xwindow},
    window::event::Event,
};

#[repr(C)]
pub struct XEvent {
    pub data: [u8; 192],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct XClientMessageEvent {
    _type: i32,
    _serial: usize,
    _send_event: i32,
    _display: *mut std::ffi::c_void,
    _window: usize,
    message_type: usize,
    format: i32,
    data: [u64; 5],
}

impl XClientMessageEvent {
    pub fn get_data(&self) -> [u64; 5] {
        self.data
    }
}

impl Event {
    fn decode_client_message(event: &XEvent, atoms: &atoms::Atoms) -> Event {
        unsafe {
            let atom = *(event.data.as_ptr() as *const XClientMessageEvent);
            if atom.get_data()[0] == atoms.wm_delete_window {
                return Event::Quit;
            }
            Event::ClientMessage
        }
    }

    fn _decode_key_press(_event: &XEvent) -> Event {
        Event::KeyPress { keycode: 0 }
    }

    pub fn decode_event(window: &xwindow::X11Window) -> Event {
        let atoms = window.get_atoms();
        let event = window.next_event();
        unsafe {
            let event_type = *(event.data.as_ptr() as *const i32);
            match event_type {
                33 => Event::decode_client_message(&event, &atoms),
                _ => Event::None,
            }
        }
    }
}
