use tokio::sync::mpsc;

use bevy::prelude::Resource;

use crate::grpc::{PeerHeartbeatReq, PeerHeartbeatRes};

pub enum BevyMessage {
    Heartbeat(PeerHeartbeatReq),
}

pub enum TonicMessage {
    Heartbeat(PeerHeartbeatRes),
}

// this would be how it looks on the bevy side
#[derive(Resource)]
pub struct TonicLink {
    pub sender: mpsc::Sender<BevyMessage>,
    pub receiver: mpsc::Receiver<TonicMessage>,
}

pub struct BevyLink {
    pub sender: mpsc::Sender<TonicMessage>,
    pub receiver: mpsc::Receiver<BevyMessage>,
}

// impl Bridge {
//     pub fn send_message_to_tonic(&self, message: BevyMessage) -> Result<()> {
//         match self.sender.try_send(message) {
//             Ok(_) => Ok(()),
//             Err(err) => Err(eyre!(err)),
//         }
//     }
//     pub fn receive_message_from_tonic(&mut self) -> Option<TonicMessage> {
//         match self.receiver.try_recv() {
//             Ok(message) => Some(message),
//             Err(_) => None,
//         }
//     }
// }
