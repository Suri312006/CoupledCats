use tokio::sync::mpsc;

use crate::grpc::{P2pHeartbeatReq, P2pHeartbeatRes};

pub enum DaemonMessage {
    Heartbeat(P2pHeartbeatReq),
}

pub enum ClientMessage {
    Heartbeat(P2pHeartbeatRes),
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
