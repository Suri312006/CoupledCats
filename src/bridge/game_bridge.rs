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

