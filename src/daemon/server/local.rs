use crate::{
    bridge::ClientLink,
    grpc::{local_server::Local, LocalHeartbeatReq, LocalHeartbeatRes},
};
use log::trace;
use tonic::{Request, Response, Status};

#[derive(Debug)]
pub struct MyLocalServer {
    pub client: ClientLink,
}

#[tonic::async_trait]
impl Local for MyLocalServer {
    async fn heartbeat(
        &self,
        request: Request<LocalHeartbeatReq>,
    ) -> Result<Response<LocalHeartbeatRes>, Status> {
        println!("request received!!!");
        trace!("Heartbeat recieved from {}", request.into_inner().name);
        Ok(Response::new(LocalHeartbeatRes {
            reply: "success".into(),
        }))
    }
}
