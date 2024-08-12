use std::net::SocketAddr;

use color_eyre::eyre::Result;
use log::error;
use tonic::transport::{server::Router, Server};

use crate::{
    bridge::ClientLink,
    grpc::{local_server::LocalServer, peer_server::PeerServer},
};

use super::server::{local::MyLocalServer, peer::MyPeerServer};

pub struct Daemon {
    addr: SocketAddr,
    router: Router,
}

impl Daemon {
    pub async fn new(client_link: ClientLink, addr: SocketAddr) -> Self {
        let router = Server::builder()
            .add_service(LocalServer::new(MyLocalServer {}))
            .add_service(PeerServer::new(MyPeerServer {
                client: client_link,
            }));

        Daemon { router, addr }
    }
    pub async fn run(self) {
        match self.router.serve(self.addr).await {
            Ok(_) => {}
            Err(err) => error!("{err:#?}"),
        }
    }
}
