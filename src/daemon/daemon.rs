use std::net::SocketAddr;

use tonic::transport::Server;

use crate::{
    bridge::ClientLink,
    grpc::{local_server::LocalServer, p2p_server::P2pServer},
};

use super::server::{local::MyLocalServer, p2p::MyP2PServer};

pub struct Daemon {
    pub local: MyLocalServer,
    pub peer: MyP2PServer,
}

impl Daemon {
    pub async fn run(client_link: ClientLink, addr: SocketAddr) {
        let result = Server::builder()
            .add_service(LocalServer::new(MyLocalServer {
                client: client_link,
            }))
            .add_service(P2pServer::new(MyP2PServer {}))
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
