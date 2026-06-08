use steel::window::{Window, event::Event};

fn main() {
    let window = Window::new();
    loop {
        let event = window.next_event();
        if let Event::Quit = event {
            break;
        }
    }
}
