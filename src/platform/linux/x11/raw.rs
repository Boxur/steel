use crate::platform::linux::x11::xevent::Data;

pub type XDisplay = std::ffi::c_void;
pub type XWindow = usize;

#[link(name = "X11")]
unsafe extern "C" {
    pub fn XOpenDisplay(name: *const i8) -> *mut XDisplay;
    pub fn XCloseDisplay(display: *mut XDisplay) -> i32;
    pub fn XDefaultScreen(display: *mut XDisplay) -> i32;
    pub fn XRootWindow(display: *mut XDisplay, screen_number: i32) -> XWindow;
    pub fn XMapWindow(display: *mut XDisplay, window: XWindow) -> i32;
    pub fn XSelectInput(display: *mut XDisplay, window: XWindow, event_mask: isize) -> i32;
    pub fn XNextEvent(display: *mut XDisplay, event: &mut Data);
    pub fn XInternAtom(
        display: *mut XDisplay,
        name: *const std::ffi::c_char,
        only_if_exists: i32,
    ) -> usize;
    pub fn XCreateSimpleWindow(
        display: *mut XDisplay,
        parent: XWindow,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        border_width: u32,
        border: usize,
        background: usize,
    ) -> XWindow;
}
