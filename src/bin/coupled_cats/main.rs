use std::net::SocketAddr;

use bevy::prelude::*;
use color_eyre::eyre::Result;
use coupled_cats::{
    daemon::daemon::MyGreeter,
    grpc::{
        hello_service_client::HelloServiceClient, hello_service_server::HelloServiceServer,
        HelloReq,
    },
    BevyMessage, Bridge, CoupledCats, TonicMessage,
};
use tokio::sync::mpsc;
use tonic::{transport::Server, IntoRequest};

#[tokio::main]
async fn main() -> Result<()> {
    // create bridge
    let (bevy_sender, mut tonic_receiver) = mpsc::channel::<BevyMessage>(100);
    let (tonic_sender, mut bevy_receiver) = mpsc::channel::<TonicMessage>(100);

    let daemon_addr: SocketAddr = "[::1]:50051".parse()?;

    // spawn daemon
    tokio::spawn(async move {
        let result = Server::builder()
            .add_service(HelloServiceServer::new(MyGreeter {}))
            .serve(daemon_addr.clone())
            .await;
        match result {
            Ok(_) => {}
            Err(err) => eprintln!("{}", color_eyre::eyre::format_err!(err)),
        }
    });

    // spawn client?
    tokio::spawn(async move {
        if let Ok(mut client) =
            HelloServiceClient::connect(format!("http://{}", daemon_addr.clone())).await
        {
            while let Some(message) = tonic_receiver.recv().await {
                match message {
                    BevyMessage::HelloReq(hello_req) => {
                        match client.say_hello(hello_req.into_request()).await {
                            Ok(resp) => {
                                match tonic_sender
                                    .try_send(TonicMessage::HelloRes(resp.into_inner()))
                                {
                                    Ok(_) => {}
                                    Err(e) => {
                                        eprintln!("Error sending message: {}", e);
                                    }
                                }
                            }

                            Err(err) => {
                                println!("Error sending message: {}", err);
                            }
                        }
                    }
                }
            }
        }
    });

    let mut app = App::new();

    app.add_systems(Update, send_message_to_tonic);
    app.add_systems(Update, receive_message_from_tonic);
    app.insert_resource(Bridge {
        sender: bevy_sender,
        receiver: bevy_receiver,
    });

    // eventually wanna pass bridge into here
    CoupledCats::run(app);
    Ok(())
}

fn send_message_to_tonic(bridge: Res<Bridge>) {
    bridge.send_message_to_tonic(BevyMessage::HelloReq(HelloReq {
        name: "Bevy".to_string(),
    }));
}

fn receive_message_from_tonic(mut bridge: ResMut<Bridge>) {
    if let Some(message) = bridge.receive_message_from_tonic() {
        match message {
            TonicMessage::HelloRes(reply) => {
                println!("Received message from server: {}", reply.reply);
            }
        }
    }
}
