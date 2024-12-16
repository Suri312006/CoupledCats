use crate::bridge::{
    clientlink::{BevyClientMessage, ClientLink, ClientMessage},
    serverlink::{BevyDaemonMessage, DaemonLink, DaemonMessage},
};
use bevy::{prelude::*, reflect::List};
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

impl Default for HeartbeatBundle {
    fn default() -> Self {
        HeartbeatBundle {
            stack: HeartbeatStack(vec![()]),
            timer: HeartbeatTimer(Timer::from_seconds(10.0, TimerMode::Repeating)),
        }
    }
}

// we want to listen for these events and handle them accordingly
pub fn check_peer_heartbeat(
    time: Res<Time>,
    mut query: Query<(&mut HeartbeatStack, &mut HeartbeatTimer)>,
    link: ResMut<ClientLink>,
) {
    for (mut stack, mut timer) in &mut query {
        timer.tick(time.delta());

        if stack.0.len() > 10 {
            error!("Other Cat dead!!");
            return;
        }

        if timer.just_finished() {
            stack.0.push(());
            match link.0.sender.try_send(BevyClientMessage::HeartbeatReq) {
                Err(TrySendError::Closed(_)) => error!("sender channel closed"),
                _ => {}
            };
        }
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

pub fn receive_heartbeat(
    mut query: Query<&mut HeartbeatStack>,
    mut network_tick: EventReader<NetworkEvent>,
) {
    for event in network_tick.read() {
        match event.0 {
            ClientMessage::HeartbeatRes => {
                trace!("Received heartbeat response!");
                let mut stack = query
                    .get_single_mut()
                    .expect("unable to access heartbeat stack");

                stack.0.pop();
            }
            _ => {}
        }
    }
}
