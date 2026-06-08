use steel::platform::linux::x11::{
    X11Window,
    raw::{XEvent, XNextEvent},
};

fn main() {
    let window = X11Window::new();
    loop {
        window.next_event();
    }
}
