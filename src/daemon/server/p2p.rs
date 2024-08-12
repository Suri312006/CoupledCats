use log::trace;
use tonic::{Request, Response, Status};

use crate::grpc::{p2p_server::P2p, P2pHeartbeatReq, P2pHeartbeatRes};

#[derive(Debug)]
pub struct P2PServer;

#[tonic::async_trait]
impl P2p for P2PServer {
    async fn heartbeat(
        &self,
        request: Request<P2pHeartbeatReq>,
    ) -> Result<Response<P2pHeartbeatRes>, Status> {
        trace!("Heartbeat recieved from {}", request.into_inner().name);
        Ok(Response::new(P2pHeartbeatRes {
            reply: "success".into(),
        }))
    }
}
