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

// https://wiki.vg/Chunk_Format
impl Default for ChunkDataUpdateLight {
    fn default() -> Self {
        Self {
            chunk_x: 0,
            chunk_z: 0,
            heightmaps: Nbt::new("", {
                let mut map = HashMap::new();
                map.insert("MOTION_BLOCKING", Tag::List(vec![]));

                map
            }),
            data: vec![],
            block_entities: vec![],
            sky_light_mask: BitSet::new(1),
            block_light_mask: BitSet::new(1),
            empty_sky_light_mask: BitSet::new(1),
            empty_block_light_mask: BitSet::new(1),
            sky_lights: vec![],
            block_lights: vec![],
        }
    }
}

#[derive(Streamable)]
#[packet_id(0x54)]
pub struct SetDefaultSpawnPosition {
    location: Position,
    angle: f32,
}

impl Default for SetDefaultSpawnPosition {
    fn default() -> Self {
        Self {
            angle: 0f32,
            location: Position::new(0.0, 0.0, 0.0),
        }
    }
}

#[derive(Streamable)]
#[packet_id(0x3E)]
pub struct SynchronizePlayerPosition {
    x: f64,
    y: f64,
    z: f64,
    yaw: f32,
    pitch: f32,
    flags: u8,
    teleport_id: VarInt,
}

impl Default for SynchronizePlayerPosition {
    fn default() -> Self {
        Self {
            x: 123.0,
            y: 0.0,
            z: 456.0,
            yaw: 0.0,
            pitch: 0.0,
            flags: 255,
            teleport_id: VarInt(0),
        }
    }
}
