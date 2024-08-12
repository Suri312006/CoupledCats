use std::{net::SocketAddr, str::FromStr};

use bevy::prelude::*;
use color_eyre::eyre::Result;
use coupled_cats::{
    bridge::{
        BevyLink, BevyMessage, ClientLink, ClientMessage, DaemonLink, DaemonMessage, TonicLink,
        TonicMessage,
    },
    Client,
    grpc::PeerHeartbeatReq,
    utils::meow,
    CoupledCats, Daemon,
};
use log::error;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<()> {
    meow::setup()?;

    //NOTE: create the links
    let (tonic_sender, tonic_receiver) = mpsc::channel::<TonicMessage>(100);
    let (bevy_sender, bevy_receiver) = mpsc::channel::<BevyMessage>(100);

    let (client_sender, client_receiver) = mpsc::channel::<ClientMessage>(100);
    let (daemon_sender, daemon_receiver) = mpsc::channel::<DaemonMessage>(100);

    tokio::spawn(async move {
        let daemon = Daemon::new(
            ClientLink {
                sender: daemon_sender,
                receiver: client_receiver,
            },
            SocketAddr::from_str("[::1]:50051").expect("wrong socket addr"),
        )
        .await;

        daemon.run().await;
    });

    tokio::spawn(async move {
        let client = match Client::new(
            BevyLink {
                sender: tonic_sender,
                receiver: bevy_receiver,
            },
            DaemonLink {
                sender: client_sender,
                receiver: daemon_receiver,
            },
            String::from("http://[::1]:50051"),
        )
        .await
        {
            Ok(conn) => conn,
            Err(err) => {
                error!("{err}");
                return;
            }
        };

        client.run().await;
    });

    let mut app = App::new();

    app.add_systems(Update, send_message_to_tonic);
    app.add_systems(Update, receive_message_from_tonic);
    app.insert_resource(TonicLink {
        sender: bevy_sender,
        receiver: tonic_receiver,
    });

    // eventually wanna pass bridge into here
    CoupledCats::run(app);
    Ok(())
}

fn send_message_to_tonic(bridge: Res<TonicLink>) {
    match bridge
        .sender
        .try_send(BevyMessage::Heartbeat(PeerHeartbeatReq {
            name: "Bevy".to_string(),
        })) {
        Ok(_) => {}
        Err(err) => error!("{err}"),
    }
}

fn receive_message_from_tonic(mut bridge: ResMut<TonicLink>) {
    if let Ok(message) = bridge.receiver.try_recv() {
        match message {
            TonicMessage::Heartbeat(reply) => {
                trace!("Recieved message from server: {}", reply.reply);
            }
        }
    }
}
