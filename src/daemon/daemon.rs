use std::net::SocketAddr;

use tonic::transport::Server;

use crate::{
    bridge::ClientLink,
    grpc::{local_server::LocalServer, peer_server::PeerServer},
};

use super::server::{local::MyLocalServer, peer::MyPeerServer};

pub struct Daemon {
    pub local: MyLocalServer,
    pub peer: MyPeerServer,
}

impl Daemon {
    pub async fn run(client_link: ClientLink, addr: SocketAddr) {
        let result = Server::builder()
            .add_service(LocalServer::new(MyLocalServer {}))
            .add_service(PeerServer::new(MyPeerServer {
                client: client_link,
            }))
            .serve(addr)
            .await;

        match result {
            Ok(_) => {}
            Err(err) => {
                //TODO: log this guy and move on
                eprintln!("{:#?}", err);
            }
        }
    }
}
