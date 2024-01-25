use std::net::TcpStream;
use uuid::Uuid;

pub trait Player {
    fn disconnect(reason: String) -> bool;
}

pub struct McPlayer {
    pub(crate) stream: TcpStream,
    pub username: String,
    pub uuid: Uuid,
}

impl Player for McPlayer {
    fn disconnect(reason: String) -> bool {
        todo!()
    }
}
