use crate::encoder::Encoder;
use macros::Streamable;

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
