use crate::decoder::Decoder;
use macros::Receivable;

#[derive(Receivable, Debug)]
pub struct PlayerPosition {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub on_ground: bool,
}
