use crate::encoder::Encoder;
use crate::encoder::EncoderWriteExt;
use crate::errors::EncodeError;
use crate::types::BitSet;
use crate::types::Position;
use crate::types::VarInt;
use macros::{Serializable, Streamable};
use nbt::io::Nbt;
use nbt::types::Tag;
use std::collections::HashMap;
use std::io::Write;

#[derive(Streamable)]
#[packet_id(0x25)]
pub struct ChunkDataUpdateLight {
    chunk_x: i32,
    chunk_z: i32,
    heightmaps: Nbt,
    data: Vec<u8>,
    block_entities: Vec<BlockEntity>,
    sky_light_mask: BitSet,
    block_light_mask: BitSet,
    empty_sky_light_mask: BitSet,
    empty_block_light_mask: BitSet,
    sky_lights: Vec<Light>,
    block_lights: Vec<Light>,
}

#[derive(Serializable, Clone)]
pub struct BlockEntity {
    packed_xz: u8,
    y: i16,
    typee: VarInt,
    data: Nbt,
}

#[derive(Serializable, Clone)]
pub struct Light {
    length: VarInt,
    sky_light_array: [u8; 2048],
}

impl Default for ChunkDataUpdateLight {
    fn default() -> Self {
        Self {
            chunk_x: 1,
            chunk_z: 1,
            heightmaps: Nbt::new("", {
                let mut map = HashMap::new();
                let long_array: Vec<Tag> = vec![];
                map.insert("MOTION_BLOCKING", Tag::List(long_array));

                map
            }),
            data: vec![],
            block_entities: vec![],
            sky_light_mask: BitSet::empty(),
            block_light_mask: BitSet::empty(),
            empty_sky_light_mask: BitSet::empty(),
            empty_block_light_mask: BitSet::empty(),
            sky_lights: vec![],
            block_lights: vec![],
        }
    }
}

// https://wiki.vg/Chunk_Format
