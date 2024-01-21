use crate::encoder::Encoder;
use crate::{
    decoder::Decoder,
    types::{ByteArray, VarInt},
};
use macros::{Receivable, Streamable};
use nbt::io::Nbt;
use std::fs::File;
use std::io::{Cursor, Read, Write};

#[derive(Streamable, Default)]
#[packet_id(0x02)]
pub struct FinishConfiguration {}

#[derive(Receivable, Debug)]
pub struct ClientInformation {
    pub locale: String,
    pub view_distance: i8,
    pub chat_mode: VarInt,
    pub chat_colors: bool,
    pub displayed_skin_parts: u8,
    pub main_hand: VarInt,
    pub enable_text_filtering: bool,
    pub allow_server_listing: bool,
}

#[derive(Receivable, Debug)]
pub struct ServerboundPluginMessage {
    pub channel: String,
    pub data: ByteArray,
}

#[derive(Streamable, Clone)]
#[packet_id(0x05)]
pub struct RegistryData {
    registry_codec: Nbt,
}

impl Default for RegistryData {
    fn default() -> Self {
        let mut f = File::open(r"D:\Phill030.de\Rust\mc_protocol\dimension_codec.nbt").unwrap();
        let mut buffer = vec![];
        f.read_to_end(&mut buffer).unwrap();
        let mut cursor = Cursor::new(buffer);
        let nbt = Nbt::from_reader(&mut cursor).unwrap();

        Self { registry_codec: nbt }
    }
}

#[derive(Receivable)]
pub struct ReceiveFinishConfiguration {}
