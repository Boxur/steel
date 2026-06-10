pub mod data;
pub mod event;
pub mod message;
pub mod mouse_pos;
pub mod size;

use crate::platform::linux::x11::xevent::XEvent;
use crate::platform::linux::x11::xmessage::XMessage;
use crate::platform::linux::x11::xwindow::X11Window;
use crate::window::data::WindowData;
use crate::window::message::Message;
use std::collections::HashMap;
use std::sync::mpsc::TryRecvError;
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread::{self, JoinHandle};

use channel::Channel;
use event::Event;
use mouse_pos::MousePos;
use size::WindowSize;

pub type KeyStates = HashMap<u32, bool>;
pub type ButtonStates = HashMap<u32, bool>;

#[derive(Debug)]
pub struct Window {
    main_update_channel: Channel<Message, Event>,
    update_loop: Option<JoinHandle<()>>,
    event_loop: Option<JoinHandle<()>>,
    data: Arc<Mutex<WindowData>>,
}

impl Window {
    pub fn new() -> Window {
        let (main_update_channel, update_main_channel) = channel::new();
        let (update_event_channel, event_update_channel) = channel::new();
        let data = Arc::new(Mutex::new(WindowData {
            key_states: KeyStates::new(),
            button_states: ButtonStates::new(),
            mouse_position: MousePos::default(),
            window_size: WindowSize::default(),
        }));

        let thread_data = Arc::clone(&data);
        let update_loop = thread::spawn(move || {
            let xatoms;
            loop {
                match update_event_channel.recv().unwrap() {
                    XEvent::Atoms(atoms) => {
                        xatoms = atoms;
                        break;
                    }
                    _ => {}
                }
            }
            //let atoms = xwindow.get_atoms();
            loop {
                match update_main_channel.try_recv() {
                    Ok(Message::Stop) | Err(TryRecvError::Disconnected) => break,
                    Err(TryRecvError::Empty) => {}
                }
                if let Ok(XEvent::Data(data)) = update_event_channel.try_recv() {
                    //= xwindow.next_event();
                    let event;
                    {
                        let window_data = &mut (*thread_data.lock().unwrap());
                        event = Event::decode_event(
                            &data,
                            &mut window_data.key_states,
                            &mut window_data.button_states,
                            &mut window_data.mouse_position,
                            &mut window_data.window_size,
                            &xatoms,
                        );
                    }
                    match event {
                        Event::None => {}
                        e => update_main_channel.send(e).unwrap(),
                    }
                }

                //thread::sleep(Duration::from_millis(1));
            }
            update_event_channel.send(XMessage::Stop).unwrap();
        });

        let event_loop = thread::spawn(move || {
            let xwindow = X11Window::new();
            event_update_channel
                .send(XEvent::Atoms(xwindow.get_atoms().clone()))
                .unwrap();
            loop {
                match event_update_channel.try_recv() {
                    Ok(XMessage::Stop) | Err(TryRecvError::Disconnected) => break,
                    _ => {}
                }
                event_update_channel
                    .send(XEvent::Data(xwindow.next_event()))
                    .unwrap_or_else(|_| {});
            }
        });
        Self {
            main_update_channel,
            update_loop: Some(update_loop),
            event_loop: Some(event_loop),
            data,
        }
    }

    pub fn next_event(&mut self) -> Event {
        self.main_update_channel.recv().unwrap()
        // if let Ok(event) = self.main_update_channel.try_recv() {
        //     return Some(event);
        // }
        // return None;
    }

    pub fn send_message(&mut self, message: Message) {
        self.main_update_channel.send(message).unwrap();
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

        if let Some(update_loop) = self.update_loop.take() {
            update_loop.join().unwrap();
        }
    }
}
