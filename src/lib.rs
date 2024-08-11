pub mod daemon;
mod game;
mod utils;

pub mod grpc {
    tonic::include_proto!("coupledcats");
}

pub use game::*;
pub use utils::*;

