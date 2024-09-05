use super::Message;

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
