use crate::platform::linux::x11::raw;

pub type Data = [u8; 192];
#[repr(C)]
pub enum XEvent {
    Data(Data),
}

type Time = usize;
type Bool = i32;

#[repr(C)]
#[derive(Clone)]
pub struct XInputEvent {
    _type: i32,
    _serial: usize,
    _send_event: Bool,
    _display: *mut std::ffi::c_void,
    _window: raw::XWindow,
    _root: raw::XWindow,
    _subwindow: raw::XWindow,
    _time: Time,
    pub x: i32,
    pub y: i32,
    pub x_root: i32,
    pub y_root: i32,
    pub state: u32,
    pub keycode: u32,
    _same_screen: Bool,
}

#[repr(C)]
#[derive(Clone)]
#[allow(dead_code)] //idk if i will ever use this but i will leave it here if i ever do
pub struct XCrossingEvent {
    _type: i32,
    _serial: usize,
    _send_event: Bool,
    _display: *mut std::ffi::c_void,
    _window: raw::XWindow,
    _root: raw::XWindow,
    _subwindow: raw::XWindow,
    _time: Time,
    pub x: i32,
    pub y: i32,
    pub x_root: i32,
    pub y_root: i32,
    pub mode: i32,
    pub detail: i32,
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
    _window: raw::XWindow,
    _mode: i32,
    detail: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct XResizeRequestEvent {
    _type: i32,
    _serial: usize,
    _sed_event: Bool,
    _display: *mut std::ffi::c_void,
    _window: raw::XWindow,
    pub width: i32,
    pub height: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct XClientMessageEvent {
    _type: i32,
    _serial: usize,
    _send_event: i32,
    _display: *mut std::ffi::c_void,
    _window: usize,
    message_type: usize,
    format: i32,
    pub data: [usize; 5],
}
