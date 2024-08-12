pub mod daemon;
mod game;
mod utils;
mod bridge;
pub mod client;

pub mod grpc {
    tonic::include_proto!("coupledcats");
}

pub use game::*;
pub use utils::*;
pub use bridge::*;
