use crate::decoder::Decoder;
use crate::types::VarInt;
use macros::Receivable;

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
