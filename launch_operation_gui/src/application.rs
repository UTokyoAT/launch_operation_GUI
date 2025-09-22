use crate::traits::{Sender, Receiver};
use crate::command_parser::CommandParser;
use crate::traits::Sendable;
use std::fmt::Debug;
use anyhow::Result;
use std::marker::PhantomData;

pub struct SendService<T: Clone + Send + Sendable + Debug, S: Sender> {
    command_parser: CommandParser<T>,
    sender: S,
}

impl<T: Clone + Send + Sendable + Debug, S: Sender> SendService<T, S> {
    pub fn new(command_parser: CommandParser<T>, sender: S) -> SendService<T, S> {
        SendService {
            command_parser,
            sender,
        }
    }

    pub async fn send(&mut self, command: String) -> Result<()> {
        let command = self.command_parser.parse(&command)?;
        self.sender.send(command).await?;
        Ok(())
    }
}


pub struct ReceiveService<T: Clone + Send + Sendable + Debug, R: Receiver, F: Fn(T)> {
    on_receive: F,
    receiver: R,
    _phantom: PhantomData<T>,
}

impl<T: Clone + Send + Sendable + Debug, R: Receiver, F: Fn(T)> ReceiveService<T, R, F> {
    pub fn new(on_receive: F, receiver: R) -> ReceiveService<T, R, F> {
        ReceiveService {
            on_receive,
            receiver,
            _phantom: PhantomData,
        }
    }

    pub async fn receive(&mut self) -> Result<T> {
        let data: T = self.receiver.receive().await?;
        (self.on_receive)(data.clone());
        Ok(data)
    }
}