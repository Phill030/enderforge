use crate::decoder::Decoder;
use macros::Receivable;

#[derive(Receivable, Debug)]
pub struct PluginMessage {
    pub channel: String,
    pub data: Vec<u8>,
}
