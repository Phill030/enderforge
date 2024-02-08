use crate::{encoder::SendToStream, packets::outgoing::play_disconnect::PlayDisconnect};
use std::{
    io,
    net::{Shutdown, TcpStream},
};
use uuid::Uuid;

pub trait Player {
    fn disconnect<S>(&mut self, reason: S) -> io::Result<()>
    where
        S: Into<String>,
        Self: Sized;
}

pub struct McPlayer {
    pub(crate) stream: TcpStream,
    pub username: String,
    pub uuid: Uuid,
}

impl Player for McPlayer {
    fn disconnect<S>(&mut self, reason: S) -> io::Result<()>
    where
        S: Into<String>,
    {
        if let Ok(_) = PlayDisconnect::from_text(reason.into()).send(&mut self.stream) {
            return Ok(self.stream.shutdown(Shutdown::Both)?);
        }

        Ok(())
    }
}

impl Clone for McPlayer {
    fn clone(&self) -> Self {
        Self {
            stream: self.stream.try_clone().unwrap(),
            username: self.username.clone(),
            uuid: self.uuid.clone(),
        }
    }
}
