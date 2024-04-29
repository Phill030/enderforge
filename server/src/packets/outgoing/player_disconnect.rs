use crate::encoder::Encoder;
use macros::Streamable;
use nbt::{io::Nbt, types::Tag};
use std::collections::HashMap;

#[derive(Streamable)]
#[packet_id(0x1B)]
pub struct PlayDisconnect {
    reason: Nbt,
}

impl PlayDisconnect {
    pub fn from_text<S>(message: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            reason: Nbt::new("", {
                let mut map = HashMap::new();
                map.insert("text", Tag::String(message.into()));

                map
            }),
        }
    }
}
