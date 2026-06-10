use crate::{
    platform::linux::x11::{
        raw,
        xatoms::{self, XAtoms},
    },
    window::{self, event::Event},
};

pub type Data = [u8; 192];
#[repr(C)]
pub enum XEvent {
    Data(Data),
    Atoms(XAtoms),
}

type Time = usize;
type Bool = i32;

#[repr(C)]
#[derive(Clone)]
struct XInputEvent {
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
#[derive(Clone)]
#[allow(dead_code)] //idk if i will ever use this but i will leave it here if i ever do
struct XCrossingEvent {
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
    mode: i32,
    detail: i32,
    _same_screen: Bool,
    _focus: Bool,
    _state: u32,
}

#[repr(C)]
#[derive(Clone)]
#[allow(dead_code)] //idk if i will ever use this but i will leave it here if i ever do
struct XFocusChangeEvent {
    _type: i32,
    _serial: usize,
    _send_event: Bool,
    _display: *mut std::ffi::c_void,
    _window: raw::Window,
    _mode: i32,
    detail: i32,
}

#[repr(C)]
#[derive(Clone)]
struct XResizeRequestEvent {
    _type: i32,
    _serial: usize,
    _sed_event: Bool,
    _display: *mut std::ffi::c_void,
    _window: raw::Window,
    width: i32,
    height: i32,
}

#[repr(C)]
#[derive(Clone)]
struct XClientMessageEvent {
    _type: i32,
    _serial: usize,
    _send_event: i32,
    _display: *mut std::ffi::c_void,
    _window: usize,
    message_type: usize,
    format: i32,
    data: [usize; 5],
}

impl XClientMessageEvent {
    pub fn get_data(&self) -> [usize; 5] {
        self.data
    }
}

impl Event {
    fn decode_key_press(data: &Data, key_states: &mut window::KeyStates) -> Event {
        let key_press_event;
        unsafe {
            key_press_event = (*(data.as_ptr() as *const XInputEvent)).clone();
        }
        *key_states.entry(key_press_event.keycode).or_insert(true) = true;
        Event::Key {
            _x: key_press_event.x,
            _y: key_press_event.y,
            state: key_press_event.state,
            keycode: key_press_event.keycode,
        }
    }

    fn decode_key_release(data: &Data, key_states: &mut window::KeyStates) -> Event {
        let key_release_event;
        unsafe {
            key_release_event = (*(data.as_ptr() as *const XInputEvent)).clone();
        }
        *key_states.entry(key_release_event.keycode).or_insert(false) = false;
        Event::Key {
            _x: key_release_event.x,
            _y: key_release_event.y,
            state: key_release_event.state,
            keycode: key_release_event.keycode,
        }
    }

    fn decode_button_press(data: &Data, button_states: &mut window::ButtonStates) -> Event {
        let button_press_event;
        unsafe {
            button_press_event = (*(data.as_ptr() as *const XInputEvent)).clone();
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

    fn decode_button_release(data: &Data, button_states: &mut window::ButtonStates) -> Event {
        let button_release_event;
        unsafe {
            button_release_event = (*(data.as_ptr() as *const XInputEvent)).clone();
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

    fn decode_motion(data: &Data, mouse_position: &mut window::mouse_pos::MousePos) -> Event {
        let motion_event;
        unsafe {
            motion_event = (*(data.as_ptr() as *const XInputEvent)).clone();
        }
        (mouse_position.x, mouse_position.y) = (motion_event.x, motion_event.y);
        Event::Motion {
            x: motion_event.x,
            y: motion_event.y,
        }
    }

    fn decode_enter_window() -> Event {
        Event::EnterNotify
    }

    fn decode_leave_window() -> Event {
        Event::LeaveNotify
    }

    fn decode_focus_in() -> Event {
        Event::FocusIn
    }

    fn decode_focus_out() -> Event {
        Event::FocusOut
    }

    fn decode_resize_request(data: &Data, window_size: &mut window::size::WindowSize) -> Event {
        unsafe {
            let resize_request_event = (*(data.as_ptr() as *const XResizeRequestEvent)).clone();
            (window_size.width, window_size.height) =
                (resize_request_event.width, resize_request_event.height);
            Event::ResizeRequest {
                width: resize_request_event.width,
                height: resize_request_event.height,
            }
        }
    }

    fn decode_client_message(data: &Data, atoms: &xatoms::XAtoms) -> Event {
        unsafe {
            let client_message_event = (*(data.as_ptr() as *const XClientMessageEvent)).clone();
            if client_message_event.get_data()[0] == atoms.wm_delete_window {
                return Event::Quit;
            }
            Event::ClientMessage
        }
    }

    pub fn decode_event(
        data: &Data,
        key_states: &mut window::KeyStates,
        button_states: &mut window::ButtonStates,
        mouse_position: &mut window::mouse_pos::MousePos,
        window_size: &mut window::size::WindowSize,
        xatoms: &xatoms::XAtoms,
    ) -> Event {
        unsafe {
            let event_type = *(data.as_ptr() as *const i32);
            match event_type {
                02 => Event::decode_key_press(&data, key_states),
                03 => Event::decode_key_release(&data, key_states),
                04 => Event::decode_button_press(&data, button_states),
                05 => Event::decode_button_release(&data, button_states),
                06 => Event::decode_motion(&data, mouse_position),
                07 => Event::decode_enter_window(),
                08 => Event::decode_leave_window(),
                09 => Event::decode_focus_in(),
                10 => Event::decode_focus_out(),
                25 => Event::decode_resize_request(&data, window_size),
                33 => Event::decode_client_message(&data, &xatoms),
                _ => Event::None,
            }
        }
    }
}
