use std::{net::SocketAddr, str::FromStr, thread::sleep, time::Duration};

use bevy::prelude::*;
use color_eyre::eyre::Result;
use coupled_cats::{
    bridge::{
        clientlink::{BevyClientMessage, ClientLink, ClientMessage},
        serverlink::{BevyDaemonMessage, DaemonLink, DaemonMessage},
        Bridge,
    },
    utils::meow,
    Client, CoupledCats, Daemon,
};
use log::error;
use tokio::sync::mpsc::{self, error::TryRecvError};

#[tokio::main]
async fn main() -> Result<()> {
    meow::setup()?;

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
            "http://[::1]:50051".into(), // would have to get this address from matchmaker
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

    app.insert_resource(ClientLink(Bridge::new(bevy_client_sender, client_receiver)));
    app.insert_resource(DaemonLink(Bridge::new(bevy_daemon_sender, daemon_receiver)));

    // eventually wanna pass bridge into here
    CoupledCats::run(app);
    Ok(())
}
