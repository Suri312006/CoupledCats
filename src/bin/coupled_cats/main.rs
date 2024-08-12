use core::panic;
use std::{net::SocketAddr, str::FromStr};

use bevy::{log::error, prelude::*};
use color_eyre::eyre::Result;
use coupled_cats::{
    client::client::Client,
    daemon::daemon::Daemon,
    grpc::{p2p_client::P2pClient, p2p_server::P2pServer},
    BevyLink, BevyMessage, ClientLink, ClientMessage, DaemonLink, DaemonMessage, TonicLink,
    TonicMessage,
};
use tokio::sync::mpsc;
use tonic::{transport::Server, IntoRequest};

#[tokio::main]
async fn main() -> Result<()> {
    // create bridge
    // let (bevy_sender, mut tonic_receiver) = mpsc::channel::<BevyMessage>(100);
    // let (tonic_sender, mut bevy_receiver) = mpsc::channel::<TonicMessage>(100);
    //
    // let daemon_addr: SocketAddr = "[::1]:50051".parse()?;
    //NOTE: create the links
    let (bevy_sender, mut bevy_receiver) = mpsc::channel::<BevyMessage>(100);
    let (tonic_sender, mut tonic_receiver) = mpsc::channel::<TonicMessage>(100);

    let (client_sender, mut client_receiver) = mpsc::channel::<ClientMessage>(100);
    let (daemon_sender, mut daemon_receiver) = mpsc::channel::<DaemonMessage>(100);

    // spawn daemon
    tokio::spawn(async move {
        Daemon::run(
            ClientLink {
                sender: daemon_sender,
                receiver: client_receiver,
            },
            SocketAddr::from_str("something").expect("wrong socket addr"),
        )
    });

    // spawn client
    tokio::spawn(async move {
        let client = Client {
            bevy: BevyLink {
                sender: tonic_sender,
                receiver: bevy_receiver,
            },
            daemon: DaemonLink {
                sender: client_sender,
                receiver: daemon_receiver,
            },
            peer: match P2pClient::connect("").await {
                Ok(conn) => conn,
                Err(err) => {
                    //TODO: log and print
                    println!("{:#?}", err);
                    return;
                }
            },
        };

        client.run().await;
    });

    let mut app = App::new();

    // app.add_systems(Update, send_message_to_tonic);
    // app.add_systems(Update, receive_message_from_tonic);
    app.insert_resource(TonicLink {
        sender: bevy_sender,
        receiver: tonic_receiver,
    });

    // eventually wanna pass bridge into here
    // CoupledCats::run(app);
    Ok(())
}

fn send_message_to_tonic(bridge: Res<TonicLink>) {
    match bridge.send_message_to_tonic(BevyMessage::HelloReq(HelloReq {
        name: "Bevy".to_string(),
    })) {
        Ok(_) => {}
        Err(err) => error!("{:#?}", err),
    }
}

fn receive_message_from_tonic(mut bridge: ResMut<TonicLink>) {
    if let Some(message) = bridge.receiver.try_recv() {
        match message {
            TonicMessage::Heartbeat(reply) => {
                println!("Received message from server: {}", reply.reply);
            }
        }
    }
}
