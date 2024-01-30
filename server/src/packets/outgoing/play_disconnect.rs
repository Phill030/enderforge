use crate::encoder::Encoder;
use macros::Streamable;
use nbt::{io::Nbt, types::Tag};
use std::{collections::HashMap, io::Write};

#[derive(Streamable)]
#[packet_id(0x1B)]
pub struct Disconnect {
    reason: Nbt, // TODO
}

impl Default for Disconnect {
    fn default() -> Self {
        Self {
            reason: Nbt::new("", {
                let mut map = HashMap::new();
                map.insert("test", Tag::String("text".to_string()));

                map
            }),
        }
    }
}
