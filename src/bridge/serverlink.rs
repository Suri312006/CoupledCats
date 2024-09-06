use bevy::prelude::Resource;

use super::{Bridge, Message};
#[derive(Resource)]
pub struct DaemonLink(pub Bridge<BevyDaemonMessage, DaemonMessage>);

#[derive(Debug, Clone)]
pub enum DaemonMessage {
    Heartbeat,
}

#[derive(Debug, Clone)]
pub enum BevyDaemonMessage {
    Heartbeat,
}

impl Message for DaemonMessage {}
impl Message for BevyDaemonMessage {}
