use bevy::prelude::Resource;

use super::{Bridge, Message};

pub enum ClientMessage {
    HeartbeatRes,
}

#[derive(Debug)]
pub enum BevyClientMessage {
    HeartbeatReq,
}

impl Message for ClientMessage {}
impl Message for BevyClientMessage {}

#[derive(Resource)]
pub struct ClientLink(pub Bridge<BevyClientMessage, ClientMessage>);
