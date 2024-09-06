use bevy::prelude::Resource;
use tokio::sync::mpsc;
pub mod clientlink;
pub mod serverlink;

pub trait Message {}

#[derive(Debug, Resource)]
pub struct Bridge<S, R>
where
    S: Message + Sync,
    R: Message + Sync,
{
    pub sender: mpsc::Sender<S>,
    pub receiver: mpsc::Receiver<R>,
}

impl<S, R> Bridge<S, R>
where
    S: Message + Send + Sync + 'static,
    R: Message + Send + Sync,
{
    // pub fn send(&mut self, message: S) -> Result<R> {
    //     // this is where we send
    //     self.sender.try_send(message);
    //
    //     for _ in 0..100 {
    //         // checking for response here
    //         match self.receiver.try_recv() {
    //             Ok(res) => return Ok(res),
    //
    //             Err(mpsc::error::TryRecvError::Disconnected) => {
    //                 bail!(mpsc::error::TryRecvError::Disconnected)
    //             }
    //
    //             _ => {}
    //         }
    //     }
    //
    //     bail!("Failed to receive message for 100 polls")
    // }

    pub fn new(sender: mpsc::Sender<S>, receiver: mpsc::Receiver<R>) -> Self {
        Bridge { sender, receiver }
    }
}
