// this client should connect to the other peers server server

use tonic::transport::Channel;

use crate::{
    bridge::{BevyLink, DaemonLink},
    grpc::p2p_client::P2pClient,
    BevyMessage, TonicMessage,
};

pub struct Client {
    // we want a channel here from our server
    pub peer: P2pClient<Channel>,

    pub bevy: BevyLink,

    pub daemon: DaemonLink,
}

impl Client {
    pub async fn run(mut self) {
        while let Some(message) = self.bevy.receiver.recv().await {
            match message {
                BevyMessage::Heartbeat(req) => {
                    match self.peer.heartbeat(req).await {
                        Ok(resp) => {
                            match self
                                .bevy
                                .sender
                                .try_send(TonicMessage::Heartbeat(resp.into_inner()))
                            {
                                Ok(_) => {}
                                Err(err) => {
                                    //TODO: log this guy
                                    eprintln!("{}", err);
                                }
                            }
                        }
                        Err(err) => {
                            //TODO: log this guy
                            eprintln!("LMAO:{}", err);
                        }
                    }
                }
            }
        }
    }
}
