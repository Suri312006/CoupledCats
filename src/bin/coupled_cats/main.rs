use std::{net::SocketAddr, str::FromStr, thread::sleep, time::Duration};

use bevy::prelude::*;
use color_eyre::eyre::Result;
use coupled_cats::{
    bridge::{
        clientlink::{BevyClientMessage, ClientMessage},
        serverlink::{BevyDaemonMessage, DaemonMessage},
        Bridge,
    },
    grpc::PeerHeartbeatReq,
    utils::meow,
    Client, CoupledCats, Daemon,
};
use log::error;
use tokio::sync::mpsc::{self, error::TryRecvError};

#[tokio::main]
async fn main() -> Result<()> {
    meow::setup()?;

    //NOTE: create the links
    // let (tonic_sender, tonic_receiver) = mpsc::channel::<TonicMessage>(100);
    // let (bevy_sender, bevy_receiver) = mpsc::channel::<BevyMessage>(100);
    //
    let (client_sender, client_receiver) = mpsc::channel::<ClientMessage>(100);
    let (daemon_sender, daemon_receiver) = mpsc::channel::<DaemonMessage>(100);
    let (bevy_client_sender, bevy_client_receiver) = mpsc::channel::<BevyClientMessage>(100);
    let (bevy_daemon_sender, bevy_daemon_receiver) = mpsc::channel::<BevyDaemonMessage>(100);

    tokio::spawn(async move {
        let daemon = Daemon::new(
            Bridge::new(daemon_sender, bevy_daemon_receiver),
            SocketAddr::from_str("[::1]:50051").expect("Weird ahh socket"),
        )
        .await;
        daemon.run().await;
    });

    tokio::spawn(async move {
        sleep(Duration::from_secs(2));
        let client = match Client::new(
            Bridge::new(client_sender, bevy_client_receiver),
            "http://[::1]:50051".into(),
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

    app.add_systems(Update, send_heartbeat);
    app.add_systems(Update, receive_heartbeat);
    app.add_systems(Update, reply_heartbeat);
    // app.add_systems(Update, receive_message_from_tonic);
    app.insert_resource(ClientLink(Bridge::new(bevy_client_sender, client_receiver)));
    app.insert_resource(DaemonLink(Bridge::new(bevy_daemon_sender, daemon_receiver)));

    // eventually wanna pass bridge into here
    CoupledCats::run(app);
    Ok(())
}

#[derive(Resource)]
struct ClientLink(Bridge<BevyClientMessage, ClientMessage>);

#[derive(Resource)]
struct DaemonLink(Bridge<BevyDaemonMessage, DaemonMessage>);

fn send_heartbeat(bridge: Res<ClientLink>) {
    match bridge.0.sender.try_send(BevyClientMessage::HeartbeatReq) {
        Ok(_) => {
            info!("Heartbeat sent");
        }
        Err(err) => {
            error!("{err}");
        }
    }
}

fn receive_heartbeat(mut bridge: ResMut<ClientLink>) {
    match bridge.0.receiver.try_recv() {
        Ok(res) => match res {
            ClientMessage::HeartbeatRes(reply) => {
                info!("{reply}")
            }
        },

        Err(TryRecvError::Disconnected) => error!("disconnected"),
        Err(TryRecvError::Empty) => {}
    }
}

fn reply_heartbeat(mut bridge: ResMut<DaemonLink>) {
    match bridge.0.receiver.try_recv() {
        Ok(msg) => match msg {
            DaemonMessage::Heartbeat => {
                match bridge.0.sender.try_send(BevyDaemonMessage::Heartbeat) {
                    Ok(_) => trace!("sent heartbeat"),
                    Err(mpsc::error::TrySendError::Closed(_msg)) => error!("Channel disconnected"),
                    _ => {}
                }
            }
        },

        Err(TryRecvError::Disconnected) => error!("Channel disconnected"),
        _ => {}
    }
}

// fn send_message_to_tonic(bridge: Res<TonicLink>) {
//     match bridge
//         .sender
//         .try_send(BevyMessage::Heartbeat(PeerHeartbeatReq {
//             name: "Bevy".to_string(),
//         })) {
//         Ok(_) => {}
//         Err(err) => error!("{err}"),
//     }
// }
//
// fn receive_message_from_tonic(mut bridge: ResMut<TonicLink>) {
//     if let Ok(message) = bridge.receiver.try_recv() {
//         match message {
//             TonicMessage::Heartbeat(reply) => {
//                 trace!("Recieved message from server: {}", reply.reply);
//             }
//         }
//     }
// }
