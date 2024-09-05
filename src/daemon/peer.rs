use std::sync::Arc;

use log::{error, trace};
use tokio::sync::Mutex;
use tonic::{Request, Response, Status};

use crate::{
    bridge::{
        serverlink::{BevyDaemonMessage, DaemonMessage},
        Bridge,
    },
    grpc::{peer_server::Peer, PeerHeartbeatReq, PeerHeartbeatRes},
};

#[derive(Debug)]
pub struct MyPeerServer {
    // holy first usage of arc mutex
    link: Arc<Mutex<Bridge<DaemonMessage, BevyDaemonMessage>>>,
}

impl MyPeerServer {
    pub fn new(link: Bridge<DaemonMessage, BevyDaemonMessage>) -> Self {
        MyPeerServer {
            link: Arc::new(Mutex::new(link)),
        }
    }
}

#[tonic::async_trait]
impl Peer for MyPeerServer {
    async fn heartbeat(
        &self,
        request: Request<PeerHeartbeatReq>,
    ) -> Result<Response<PeerHeartbeatRes>, Status> {
        trace!("Heartbeat recieved from {}", request.into_inner().name);

        match self
            .link
            .as_ref()
            .lock()
            .await
            .sender
            .send(DaemonMessage::Heartbeat)
            .await
        {
            Ok(_) => {}
            Err(err) => {
                error!("{err:#?}");
                return Err(Status::new(
                    tonic::Code::Internal,
                    "Daemon Message failed to send",
                ));
            }
        };

        match self.link.as_ref().lock().await.receiver.recv().await {
            Some(res) => match res {
                BevyDaemonMessage::Heartbeat => Ok(Response::new(PeerHeartbeatRes {
                    reply: "success".into(),
                })),
            },
            None => Ok(Response::new(PeerHeartbeatRes {
                reply: "died".into(),
            })),
        }
    }
}
