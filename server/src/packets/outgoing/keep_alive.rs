use crate::encoder::{Encoder, SendToStream};
use macros::Streamable;
use tokio::{io::AsyncWriteExt, net::tcp::WriteHalf};

#[derive(Streamable)]
#[packet_id(0x24)]
pub struct KeepAlive {
    id: i64,
}

impl KeepAlive {
    pub fn new(id: i64) -> Self {
        Self { id }
    }
}
