use log::trace;
use tonic::{Request, Response, Status};

use crate::{
    grpc::{peer_server::Peer, PeerHeartbeatReq, PeerHeartbeatRes},
    ClientLink,
};

#[derive(Debug)]
pub struct MyPeerServer {
    pub client: ClientLink,
}

#[tonic::async_trait]
impl Peer for MyPeerServer {
    async fn heartbeat(
        &self,
        request: Request<PeerHeartbeatReq>,
    ) -> Result<Response<PeerHeartbeatRes>, Status> {
        trace!("Heartbeat recieved from {}", request.into_inner().name);
        Ok(Response::new(PeerHeartbeatRes {
            reply: "success".into(),
        }))
    }
}
