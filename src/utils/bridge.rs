use color_eyre::eyre::{eyre, Result};
use tokio::sync::mpsc;

use bevy::prelude::Resource;

use crate::grpc::{HelloReq, HelloRes};

pub enum BevyMessage {
    HelloReq(HelloReq),
}

pub enum TonicMessage {
    HelloRes(HelloRes),
}

#[derive(Resource)]
pub struct Bridge {
    pub sender: mpsc::Sender<BevyMessage>,
    pub receiver: mpsc::Receiver<TonicMessage>,
}

impl Bridge {
    pub fn send_message_to_tonic(&self, message: BevyMessage) -> Result<()> {
        match self.sender.try_send(message) {
            Ok(_) => Ok(()),
            Err(err) => Err(eyre!(err)),
        }
    }
    pub fn receive_message_from_tonic(&mut self) -> Option<TonicMessage> {
        match self.receiver.try_recv() {
            Ok(message) => Some(message),
            Err(_) => None,
        }
    }
}
