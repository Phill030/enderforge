use crate::{
    encoder::{Encoder, EncoderWriteExt},
    errors::EncodeError,
    types::{Position, VarInt},
};
use macros::{Serializable, Streamable};

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

impl Default for PlayLogin {
    fn default() -> Self {
        PlayLogin {
            entity_id: 0,
            is_hardcore: false,
            world_names: vec![
                "minecraft:overworld".into(),
                "minecraft:the_nether".into(),
                "minecraft:the_end".into(),
            ],
            max_players: VarInt(123_456),
            view_distance: VarInt(10),
            simulation_distance: VarInt(10),
            reduced_debug_info: false,
            enable_respawn_screen: true,
            do_limited_crafting: false,
            dimension_type: "minecraft:overworld".into(),
            dimension_name: "minecraft:overworld".into(),
            hashed_seed: 123456789i64,
            game_mode: 1,
            previous_game_mode: -1,
            is_debug: true,
            is_flat: false,
            death: None,
            portal_cooldown: VarInt(0),
        }
    }
}
