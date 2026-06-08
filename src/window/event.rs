use crate::platform::linux::x11::event;
use crate::window::atoms;

#[repr(C)]
#[derive(Clone, Copy)]
struct XClientMessageEvent {
    _type: i32,
    _serial: usize,
    _send_event: i32,
    _display: *mut std::ffi::c_void,
    _window: usize,
    message_type: usize,
    format: i32,
    data: [u64; 5],
}

#[derive(Debug)]
pub enum Event {
    KeyDown { keycode: u8 },
    KeyUp { keycode: u8 },
    Quit,
    None,
}

impl Event {
    fn decode_client_message(event: &event::XEvent, atoms: &atoms::Atoms) -> Event {
        unsafe {
            let atom = *(event.data.as_ptr() as *const XClientMessageEvent);
            if atom.data[0] == atoms.wm_delete_window {
                return Event::Quit;
            }
            Event::None
        }
    }

    pub fn decode_event(event: event::XEvent, atoms: &atoms::Atoms) -> Event {
        unsafe {
            let event_type = *(event.data.as_ptr() as *const i32);
            match event_type {
                33 => Event::decode_client_message(&event, atoms),
                _ => Event::None,
            }
        }
    }
}
