use crate::encoder::Encoder;
use macros::Streamable;
use tokio::io::AsyncWriteExt;

#[derive(Streamable)]
#[packet_id(0x20)]
pub struct GameEvent {
    event: u8,
    value: f32,
}

impl Default for GameEvent {
    fn default() -> Self {
        Self { event: 13, value: 0.0 }
    }
}
