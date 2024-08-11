use bevy::prelude::*;
use hello_world::greeter_client::GreeterClient;
use hello_world::{HelloReply, HelloRequest};
use tokio::sync::mpsc;

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

pub enum BevyMessage {
    HelloRequest(HelloRequest),
}

pub enum TonicMessage {
    HelloReply(HelloReply),
}

#[derive(Resource)]
pub struct TonicBridge {
    sender: mpsc::Sender<BevyMessage>,
    receiver: mpsc::Receiver<TonicMessage>,
}

impl TonicBridge {
    pub fn send_message_to_tonic(&self, message: BevyMessage) {
        match self.sender.try_send(message) {
            Ok(_) => {}
            Err(e) => {
                println!("Error sending message: {}", e);
            }
        }
    }

    pub fn receive_message_from_tonic(&mut self) -> Option<TonicMessage> {
        match self.receiver.try_recv() {
            Ok(message) => Some(message),
            Err(_) => None,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let (bevy_sender, mut tonic_receiver) = mpsc::channel::<BevyMessage>(100);
    let (tonic_sender, bevy_receiver) = mpsc::channel::<TonicMessage>(100);

    let mut app = App::new();

    tokio::spawn(async move {
        if let Ok(mut client) = GreeterClient::connect("http://[::1]:50051").await {
            while let Some(message) = tonic_receiver.recv().await {
                match message {
                    BevyMessage::HelloRequest(hello_request) => {
                        match client.say_hello(tonic::Request::new(hello_request)).await {
                            Ok(response) => {
                                match tonic_sender
                                    .try_send(TonicMessage::HelloReply(response.into_inner()))
                                {
                                    Ok(_) => {}
                                    Err(e) => {
                                        println!("Error sending message: {}", e);
                                    }
                                }
                            }
                            Err(e) => {
                                println!("Error sending message: {}", e);
                            }
                        }
                    }
                }
            }
        }
    });

    app.add_plugins(DefaultPlugins);
    app.add_systems(Update, send_message_to_tonic);
    app.add_systems(Update, receive_message_from_tonic);
    app.insert_resource(TonicBridge {
        sender: bevy_sender,
        receiver: bevy_receiver,
    });
    app.run();

    Ok(())
}

fn send_message_to_tonic(bridge: Res<TonicBridge>) {
    bridge.send_message_to_tonic(BevyMessage::HelloRequest(HelloRequest {
        name: "Bevy".to_string(),
    }));
}

fn receive_message_from_tonic(mut bridge: ResMut<TonicBridge>) {
    if let Some(message) = bridge.receive_message_from_tonic() {
        match message {
            TonicMessage::HelloReply(reply) => {
                println!("Received message from server: {}", reply.message);
            }
        }
    }
}
