use super::xevent::XEvent;

pub enum Display {}
pub type Window = usize;

#[link(name = "X11")]
unsafe extern "C" {
    pub fn XOpenDisplay(name: *const i8) -> *mut Display;
    pub fn XCloseDisplay(display: *mut Display) -> i32;
    pub fn XDefaultScreen(display: *mut Display) -> i32;
    pub fn XRootWindow(display: *mut Display, screen_number: i32) -> Window;
    pub fn XMapWindow(display: *mut Display, window: Window) -> i32;
    pub fn XSelectInput(display: *mut Display, window: Window, event_mask: isize) -> i32;
    pub fn XNextEvent(display: *mut Display, event: &mut XEvent);
    pub fn XInternAtom(
        display: *mut Display,
        name: *const std::ffi::c_char,
        only_if_exists: i32,
    ) -> usize;
    pub fn XCreateSimpleWindow(
        display: *mut Display,
        parent: Window,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        border_width: u32,
        border: usize,
        background: usize,
    ) -> Window;
}
