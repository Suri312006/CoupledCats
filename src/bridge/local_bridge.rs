use tokio::sync::mpsc;

use crate::grpc::{PeerHeartbeatReq, PeerHeartbeatRes};

pub enum DaemonMessage {
    Heartbeat(PeerHeartbeatReq),
}

pub enum ClientMessage {
    Heartbeat(PeerHeartbeatRes),
}

// this would be how it looks on the bevy side
#[derive(Debug)]
pub struct ClientLink {
    pub sender: mpsc::Sender<DaemonMessage>,
    pub receiver: mpsc::Receiver<ClientMessage>,
}

pub struct DaemonLink {
    pub sender: mpsc::Sender<ClientMessage>,
    pub receiver: mpsc::Receiver<DaemonMessage>,
}
