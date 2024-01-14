use crate::{decoder::Decoder, types::VarInt};
use macros::{Receivable, Streamable};
use std::io::Write;

#[derive(Streamable, Default)]
#[packet_id(0x02)]
pub struct FinishConfiguration {}

#[derive(Receivable)]
pub struct ClientInformation {
    locale: String,
    view_distance: i8,
    chat_mode: VarInt,
    chat_colors: bool,
    displayed_skin_parts: u8,
    main_hand: VarInt,
    enable_text_filtering: bool,
    allow_server_listing: bool,
}
