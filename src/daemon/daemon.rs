use std::net::SocketAddr;

use tonic::transport::Server;

use crate::{bridge::ClientLink, grpc::local_server::LocalServer};

use super::server::{local::MyLocalServer, p2p::P2PServer};

pub struct Daemon {
    pub local: MyLocalServer,
    pub peer: P2PServer,
}

impl Daemon {
    pub async fn run(client_link: ClientLink, addr: SocketAddr) {
        let result = Server::builder()
            .add_service(LocalServer::new(MyLocalServer {
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
