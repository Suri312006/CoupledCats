mod client; // do something with this later (rename)
mod daemon; // rename
mod game;

pub mod bridge;
pub mod utils;
pub mod grpc {
    tonic::include_proto!("coupledcats");
}

pub use daemon::*;
pub use client::*;
pub use game::*;
