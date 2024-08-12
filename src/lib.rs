mod bridge;
mod game;
mod utils;

pub mod client;
pub mod daemon;
pub mod grpc {
    tonic::include_proto!("coupledcats");
}

pub use bridge::*;
pub use game::*;
pub use utils::*;
