use super::Message;

pub enum ClientMessage {
    HeartbeatRes(String),
}

#[derive(Debug)]
pub enum BevyClientMessage {
    HeartbeatReq,
}

impl Message for ClientMessage {}
impl Message for BevyClientMessage {}
