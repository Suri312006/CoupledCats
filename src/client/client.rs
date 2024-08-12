// this client should connect to the other peers server server
use color_eyre::eyre::Result;
use log::error;
use tonic::transport::Channel;

use crate::{
    bridge::{BevyLink, BevyMessage, DaemonLink, TonicMessage},
    grpc::peer_client::PeerClient,
};

pub struct Client {
    // we want a channel here from our server
    pub peer: PeerClient<Channel>,

    pub bevy: BevyLink,

    pub daemon: DaemonLink,
}

impl Client {
    pub async fn new(
        bevy_link: BevyLink,
        daemon_link: DaemonLink,
        peer_addr: String,
    ) -> Result<Self> {
        Ok(Client {
            bevy: bevy_link,
            daemon: daemon_link,
            peer: PeerClient::connect(peer_addr).await?,
        })
    }

    pub async fn run(mut self) {
        while let Some(message) = self.bevy.receiver.recv().await {
            match message {
                BevyMessage::Heartbeat(req) => match self.peer.heartbeat(req).await {
                    Ok(resp) => {
                        match self
                            .bevy
                            .sender
                            .try_send(TonicMessage::Heartbeat(resp.into_inner()))
                        {
                            Ok(_) => {}
                            Err(err) => {
                                error!("{err}");
                            }
                        }
                    }
                    Err(err) => {
                        error!("{err}");
                    }
                },
            }
        }
    }
}
