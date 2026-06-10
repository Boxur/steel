use steel::window::{Window, event::Event, message::Message};

fn main() {
    let mut window = Window::new();
    loop {
        let event = window.next_event();
        if let Event::Quit = event {
            window.send_message(Message::Stop);
            break;
        }
        dbg!(&event);
    }
}
