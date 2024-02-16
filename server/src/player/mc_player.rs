use crate::{encoder::SendToWriter, packets::outgoing::play_disconnect::PlayDisconnect};
use std::io;
use tokio::{io::AsyncWriteExt, net::TcpStream};
use uuid::Uuid;

pub trait Player {
    async fn disconnect<S>(&mut self, reason: S) -> io::Result<bool>
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
    async fn disconnect<S>(&mut self, reason: S) -> io::Result<bool>
    where
        S: Into<String>,
    {
        if let Ok(_) = PlayDisconnect::from_text(reason.into()).send(&mut self.stream).await {
            self.stream.shutdown().await?;
            return Ok(true);
        }

        Ok(false)
    }
}

// impl Clone for McPlayer {
//     fn clone(&self) -> Self {
//         Self {
//             stream: self.stream,
//             username: self.username.clone(),
//             uuid: self.uuid.clone(),
//         }
//     }
// }
