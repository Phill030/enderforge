use crate::encoder::Encoder;
use crate::encoder::EncoderWriteExt;
use crate::errors::EncodeError;
use crate::types::Position;
use crate::types::VarInt;
use macros::{Serializable, Streamable};
use nbt::io::Nbt;
use nbt::types::Tag;
use std::collections::HashMap;
use std::io::Write;

#[derive(Streamable)]
#[packet_id(0x29)]
pub struct PlayLogin {
    pub entity_id: i32,
    pub is_hardcore: bool,
    pub world_names: Vec<String>,
    pub max_players: VarInt,
    pub view_distance: VarInt,
    pub simulation_distance: VarInt,
    pub reduced_debug_info: bool,
    pub enable_respawn_screen: bool,
    pub do_limited_crafting: bool,
    pub dimension_type: String,
    pub dimension_name: String,
    pub hashed_seed: i64,
    pub game_mode: u8,
    pub previous_game_mode: i8,
    pub is_debug: bool,
    pub is_flat: bool,
    pub death: Option<Death>,
    pub portal_cooldown: VarInt,
}

#[derive(Serializable, Clone)]
pub struct Death {
    pub dimension_name: String,
    pub location: Position,
}

// #[derive(Serializable)]
// pub enum Gamemode {
//     Survival = 1,
//     Adventure = 2,
//     Creative = 3,
// }

impl Default for PlayLogin {
    fn default() -> Self {
        let dimension_type2 = Nbt::new("minecraft:dimension_type", {
            let mut map = HashMap::new();
            map.insert("fixed_time", Tag::Long(12000));
            map.insert("has_skylight", Tag::Byte(1));
            map.insert("has_ceiling", Tag::Byte(0));
            map.insert("ultrawarm", Tag::Byte(0));
            map.insert("natural", Tag::Byte(1));
            map.insert("coordinate_scale", Tag::Double(7f64));
            map.insert("bed_works", Tag::Byte(1));
            map.insert("respawn_anchor_works", Tag::Byte(1));
            map.insert("min_y", Tag::Int(-64));
            map.insert("height", Tag::Int(384));
            map.insert("logical_height", Tag::Int(384));
            map.insert("infiniburn", Tag::String("#infiniburn_overworld".to_string()));
            map.insert("effects", Tag::String("overworld".to_string()));
            map.insert("ambient_light", Tag::Float(0.0));
            map.insert("piglin_safe", Tag::Byte(0));
            map.insert("has_raids", Tag::Byte(1));
            map.insert("monster_spawn_light_level", Tag::Int(7));
            map.insert("monster_spawn_block_light_limit", Tag::Int(0));
            map
        });

        PlayLogin {
            entity_id: 0,
            is_hardcore: false,
            world_names: vec!["minecraft:overworld".to_string()],
            max_players: VarInt(1),
            view_distance: VarInt(16),
            simulation_distance: VarInt(16),
            reduced_debug_info: false,
            enable_respawn_screen: true,
            do_limited_crafting: false,
            dimension_type: "overworld".to_string(), // Nbt (Registry)
            dimension_name: "overworld".to_string(),
            hashed_seed: 123456789i64,
            game_mode: 1,
            previous_game_mode: -1,
            is_debug: true,
            is_flat: true,
            death: None,
            portal_cooldown: VarInt(31),
        }
    }
}
