use std::sync::mpsc::{self, Receiver, SendError, Sender};

pub struct ChannelA<T, U> {
    sender: Sender<T>,
    receiver: Receiver<U>,
}
pub struct ChannelB<T, U> {
    sender: Sender<U>,
    receiver: Receiver<T>,
}

pub fn new<T, U>() -> (ChannelA<T, U>, ChannelB<T, U>) {
    let (a_sender, b_receiver) = mpsc::channel();
    let (b_sender, a_receiver) = mpsc::channel();
    (
        ChannelA {
            sender: a_sender,
            receiver: a_receiver,
        },
        ChannelB {
            sender: b_sender,
            receiver: b_receiver,
        },
    )
}

impl<T, U> ChannelA<T, U> {
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

impl<T, U> ChannelB<T, U> {
    pub fn send(&self, value: U) -> Result<(), SendError<U>> {
        self.sender.send(value)
    }

    pub fn recv(&self) -> Result<T, mpsc::RecvError> {
        self.receiver.recv()
    }

    pub fn try_recv(&self) -> Result<T, mpsc::TryRecvError> {
        self.receiver.try_recv()
    }
}
