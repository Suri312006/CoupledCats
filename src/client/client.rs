// this client should connect to the other peers server server
use color_eyre::eyre::Result;
use log::{error, trace};
use tonic::{transport::Channel, IntoRequest};

use crate::{
    bridge::{
        clientlink::{BevyClientMessage, ClientMessage},
        Bridge,
    },
    grpc::{peer_client::PeerClient, PeerHeartbeatReq},
};

pub struct Client {
    // we want a channel here from our server
    pub bevy: Bridge<ClientMessage, BevyClientMessage>,

    pub peer: PeerClient<Channel>,
}

impl Client {
    pub async fn new(
        bevy_link: Bridge<ClientMessage, BevyClientMessage>,
        peer_addr: String,
    ) -> Result<Self> {
        Ok(Client {
            bevy: bevy_link,
            peer: PeerClient::connect(peer_addr).await?,
        })
    }

    pub async fn run(mut self) {
        while let Some(message) = self.bevy.receiver.recv().await {
            trace!("received message {message:#?}");
            match message {
                BevyClientMessage::HeartbeatReq => {
                    match self
                        .peer
                        .heartbeat(
                            PeerHeartbeatReq {
                                name: "None".into(),
                            }
                            .into_request(),
                        )
                        .await
                    {
                        Ok(res) => {
                            trace!("rending heartbeat response");
                            match self.bevy.sender.send(ClientMessage::HeartbeatRes).await {
                                Ok(_) => {}
                                Err(err) => {
                                    error!("{err:#?}");
                                }
                            };
                        }
                        Err(err) => {
                            error!("{err:#?}");
                        }
                    }
                }
            }
        }
        // while let Some(message) = self.bevy.receiver.recv().await {
        //     match message {
        //         BevyDaemonMessage::Heartbeat(req) => match self.daemon.heartbeat(req).await {
        //             Ok(resp) => {
        //                 match self
        //                     .bevy
        //                     .sender
        //                     .try_send(TonicMessage::Heartbeat(resp.into_inner()))
        //                 {
        //                     Ok(_) => {}
        //                     Err(err) => {
        //                         error!("{err}");
        //                     }
        //                 }
        //             }
        //             Err(err) => {
        //                 error!("{err}");
        //             }
        //         },
        //     }
        // }
    }
}
