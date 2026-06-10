use std::sync::mpsc::{self, Receiver, SendError, Sender};

#[derive(Debug)]
pub struct Channel<T, U> {
    sender: Sender<T>,
    receiver: Receiver<U>,
}

pub fn new<T, U>() -> (Channel<T, U>, Channel<U, T>) {
    let (a_sender, b_receiver) = mpsc::channel();
    let (b_sender, a_receiver) = mpsc::channel();
    (
        Channel {
            sender: a_sender,
            receiver: a_receiver,
        },
        Channel {
            sender: b_sender,
            receiver: b_receiver,
        },
    )
}

impl<T, U> Channel<T, U> {
    pub fn send(&self, value: T) -> Result<(), SendError<T>> {
        self.sender.send(value)
    }

    pub fn recv(&self) -> Result<U, mpsc::RecvError> {
        self.receiver.recv()
    }

    pub fn try_recv(&self) -> Result<U, mpsc::TryRecvError> {
        self.receiver.try_recv()
    }
}
