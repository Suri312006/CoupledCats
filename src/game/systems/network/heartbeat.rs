use crate::bridge::{
    clientlink::{BevyClientMessage, ClientLink, ClientMessage},
    serverlink::{BevyDaemonMessage, DaemonLink, DaemonMessage},
};
use bevy::prelude::*;
use log::error;
use tokio::sync::mpsc::error::{TryRecvError, TrySendError};

use super::NetworkEvent;

#[derive(Component, Deref, DerefMut)]
pub struct HeartbeatTimer(pub Timer);

#[derive(Component)]
pub struct HeartbeatStack(pub Vec<()>);

#[derive(Bundle)]
pub struct HeartbeatBundle {
    pub stack: HeartbeatStack,
    pub timer: HeartbeatTimer,
}

// we want to listen for these events and handle them accordingly
pub fn check_peer_heartbeat(
    time: Res<Time>,
    mut query: Query<&mut HeartbeatTimer>,
    link: ResMut<ClientLink>,
) {
    let mut timer = query
        .get_single_mut()
        .expect("unable to get heartbeat timer");
    timer.tick(time.delta());

    if timer.just_finished() {
        match link.0.sender.try_send(BevyClientMessage::HeartbeatReq) {
            Err(TrySendError::Closed(_)) => error!("sender channel closed"),
            _ => {}
        };
    }
}

pub fn reply_heartbeat(mut daemon_link: ResMut<DaemonLink>) {
    match daemon_link.0.receiver.try_recv() {
        Ok(msg) => match msg {
            DaemonMessage::Heartbeat => {
                match daemon_link.0.sender.try_send(BevyDaemonMessage::Heartbeat) {
                    Ok(_) => trace!("sent heartbeat"),
                    Err(TrySendError::Closed(_msg)) => error!("Channel disconnected"),
                    _ => {}
                }
            }
        },

        Err(TryRecvError::Disconnected) => error!("Channel disconnected"),
        _ => {}
    }
}

pub fn receive_heartbeat(mut network_tick: EventReader<NetworkEvent>) {
    for event in network_tick.read() {
        match event.0 {
            ClientMessage::HeartbeatRes => {}
            _ => {}
        }
    }
}
