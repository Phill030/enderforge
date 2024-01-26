use std::net::TcpStream;
use uuid::Uuid;

pub trait Player {
    fn disconnect(&self, reason: String) -> bool;
}

pub struct McPlayer {
    pub(crate) stream: TcpStream,
    pub username: String,
    pub uuid: Uuid,
}

impl Player for McPlayer {
    fn disconnect(&self, reason: String) -> bool {
        todo!()
    }
}
