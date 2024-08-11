use tonic::{Request, Response, Status};

use crate::grpc::{hello_service_server::HelloService, HelloReq, HelloRes};

#[derive(Debug)]
pub struct MyGreeter;

#[tonic::async_trait]
impl HelloService for MyGreeter {
    async fn say_hello(&self, req: Request<HelloReq>) -> Result<Response<HelloRes>, Status> {
        println!("req recieved");
        let reply = HelloRes {
            reply: format!("you got a small dick lmao {}", req.into_inner().name),
        };

        Ok(Response::new(reply))
    }
}
