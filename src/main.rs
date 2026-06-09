use steel::window::{Window, event::Event};

fn main() {
    let mut window = Window::new();
    loop {
        let event = window.next_event();
        if let Event::Quit = event {
            break;
        }
        dbg!(&window.button_states);
    }
}
