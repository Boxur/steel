pub mod data;
pub mod event;
pub mod message;
pub mod mouse_pos;
pub mod size;

use crate::platform::linux::x11::xwindow::X11Window;
use crate::window::data::WindowData;
use crate::window::message::Message;
use std::collections::HashMap;
use std::sync::mpsc::TryRecvError;
use std::sync::{Arc, Mutex, MutexGuard, mpsc};
use std::thread::{self, JoinHandle};

use event::Event;
use mouse_pos::MousePos;
use size::WindowSize;

pub type KeyStates = HashMap<u32, bool>;
pub type ButtonStates = HashMap<u32, bool>;

#[derive(Debug)]
pub struct Window {
    message_sender: mpsc::Sender<Message>,
    event_receiver: mpsc::Receiver<Event>,
    event_loop: Option<JoinHandle<()>>,
    data: Arc<Mutex<WindowData>>,
}

impl Window {
    pub fn new() -> Window {
        let (event_sender, event_receiver) = mpsc::channel();
        let (message_sender, message_receiver) = mpsc::channel();
        let data = Arc::new(Mutex::new(WindowData {
            key_states: KeyStates::new(),
            button_states: ButtonStates::new(),
            mouse_position: MousePos::default(),
            window_size: WindowSize::default(),
        }));
        let thread_data = Arc::clone(&data);
        let event_loop = thread::spawn(move || {
            let xwindow = X11Window::new();
            loop {
                match message_receiver.try_recv() {
                    Ok(Message::Stop) | Err(TryRecvError::Disconnected) => break,
                    Err(TryRecvError::Empty) => {}
                }
                let event;
                {
                    let data = &mut (*thread_data.lock().unwrap());
                    event = Event::decode_event(
                        &xwindow,
                        &mut data.key_states,
                        &mut data.button_states,
                        &mut data.mouse_position,
                        &mut data.window_size,
                    );
                }
                if let Event::Quit = event {
                    println!("got quit signal");
                }
                event_sender.send(event).unwrap();

                //thread::sleep(Duration::from_millis(1));
            }
        });
        Self {
            event_receiver,
            message_sender,
            event_loop: Some(event_loop),
            data,
        }
    }

    pub fn next_event(&mut self) -> Option<Event> {
        if let Ok(event) = self.event_receiver.try_recv() {
            dbg!(&event);
            return Some(event);
        }
        return None;
    }

    pub fn send_message(&mut self, message: Message) {
        self.message_sender.send(message).unwrap();
    }

    pub fn get_data(&self) -> MutexGuard<'_, WindowData> {
        self.data.lock().unwrap()
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        if let Some(event_loop) = self.event_loop.take() {
            event_loop.join().unwrap();
        }
    }
}
