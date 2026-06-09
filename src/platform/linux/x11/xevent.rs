use crate::{
    platform::linux::x11::{atoms, raw, xwindow},
    window::{self, event::Event},
};

#[repr(C)]
#[derive(Debug)]
pub struct XEvent {
    pub data: [u8; 192],
}

type Time = usize;
type Bool = i32;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct XKeyEvent {
    _type: i32,
    _serial: usize,
    _send_event: Bool,
    _display: *mut std::ffi::c_void,
    _window: raw::Window,
    _root: raw::Window,
    _subwindow: raw::Window,
    _time: Time,
    x: i32,
    y: i32,
    x_root: i32,
    y_root: i32,
    state: u32,
    keycode: u32,
    _same_screen: Bool,
}

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

impl XClientMessageEvent {
    pub fn get_data(&self) -> [u64; 5] {
        self.data
    }
}

impl Event {
    fn decode_key_press(event: &XEvent, key_states: &mut window::KeyStates) -> Event {
        let key_press_event;
        unsafe {
            key_press_event = *(event.data.as_ptr() as *const XKeyEvent);
        }
        *key_states.entry(key_press_event.keycode).or_insert(true) = true;
        Event::Key {
            _x: key_press_event.x,
            _y: key_press_event.y,
            state: key_press_event.state,
            keycode: key_press_event.keycode,
        }
    }

    fn decode_key_release(event: &XEvent, key_states: &mut window::KeyStates) -> Event {
        let key_release_event;
        unsafe {
            key_release_event = *(event.data.as_ptr() as *const XKeyEvent);
        }
        *key_states.entry(key_release_event.keycode).or_insert(false) = false;
        Event::Key {
            _x: key_release_event.x,
            _y: key_release_event.y,
            state: key_release_event.state,
            keycode: key_release_event.keycode,
        }
    }

    fn decode_button_press(event: &XEvent, button_states: &mut window::ButtonStates) -> Event {
        let button_press_event;
        unsafe {
            button_press_event = *(event.data.as_ptr() as *const XKeyEvent);
            dbg!(button_press_event);
        }
        *button_states
            .entry(button_press_event.keycode)
            .or_insert(true) = true;
        Event::Mouse {
            x: button_press_event.x,
            y: button_press_event.y,
            state: button_press_event.state,
            keycode: button_press_event.keycode,
        }
    }

    fn decode_button_release(event: &XEvent, button_states: &mut window::ButtonStates) -> Event {
        let button_release_event;
        unsafe {
            button_release_event = *(event.data.as_ptr() as *const XKeyEvent);
            dbg!(button_release_event);
        }
        *button_states
            .entry(button_release_event.keycode)
            .or_insert(false) = false;
        Event::Mouse {
            x: button_release_event.x,
            y: button_release_event.y,
            state: button_release_event.state,
            keycode: button_release_event.keycode,
        }
    }

    fn decode_client_message(event: &XEvent, atoms: &atoms::Atoms) -> Event {
        unsafe {
            let client_message_event = *(event.data.as_ptr() as *const XClientMessageEvent);
            if client_message_event.get_data()[0] == atoms.wm_delete_window {
                return Event::Quit;
            }
            Event::ClientMessage
        }
    }

    pub fn decode_event(
        window: &xwindow::X11Window,
        key_states: &mut window::KeyStates,
        button_states: &mut window::ButtonStates,
    ) -> Event {
        let atoms = window.get_atoms();
        let event = window.next_event();
        unsafe {
            let event_type = *(event.data.as_ptr() as *const i32);
            match event_type {
                02 => Event::decode_key_press(&event, key_states),
                03 => Event::decode_key_release(&event, key_states),
                04 => Event::decode_button_press(&event, button_states),
                05 => Event::decode_button_release(&event, button_states),
                33 => Event::decode_client_message(&event, &atoms),
                _ => Event::None,
            }
        }
    }
}
