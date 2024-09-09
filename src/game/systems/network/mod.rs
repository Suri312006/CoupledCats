use bevy::prelude::*;
use heartbeat::{
    check_peer_heartbeat, receive_heartbeat, reply_heartbeat, HeartbeatBundle, HeartbeatStack,
    HeartbeatTimer,
};
use log::error;
use tokio::sync::mpsc::error::TryRecvError;

use crate::bridge::clientlink::{ClientLink, ClientMessage};

mod heartbeat;

#[derive(Event)]
pub struct NetworkEvent(pub ClientMessage);

pub fn init_network_systems(app: &mut App) {
    app.add_systems(Startup, setup_network)
        .add_systems(Update, emit_network_event)
        .add_systems(
            Update,
            (check_peer_heartbeat, reply_heartbeat, receive_heartbeat),
        );
}

fn setup_network(mut commands: Commands) {
    commands.spawn(HeartbeatBundle::default());
}

fn emit_network_event(mut writer: EventWriter<NetworkEvent>, mut link: ResMut<ClientLink>) {
    match link.0.receiver.try_recv() {
        Ok(msg) => {
            writer.send(NetworkEvent(msg));
        }
        Err(TryRecvError::Disconnected) => {
            error!("Client link channel disconnected!!");
        }
        _ => {}
    };
}
