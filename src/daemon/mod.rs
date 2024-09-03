mod peer;

use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use color_eyre::eyre::Result;
use log::error;
use peer::MyPeerServer;
use tonic::transport::{server::Router, Server};

use crate::{
    bridge::{
        serverlink::{BevyDaemonMessage, DaemonMessage},
        Bridge,
    },
    grpc::peer_server::PeerServer,
};

pub struct Daemon {
    addr: SocketAddr,
    router: Router,
}

impl Daemon {
    pub async fn new(link: Bridge<DaemonMessage, BevyDaemonMessage>, addr: SocketAddr) -> Self {
        let router = Server::builder().add_service(PeerServer::new(MyPeerServer::new(link)));

        Daemon { router, addr }
    }
    pub async fn run(self) {
        match self.router.serve(self.addr).await {
            Ok(_) => {}
            Err(err) => error!("{err:#?}"),
        }
    }
}
