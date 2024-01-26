use crate::decoder::Decoder;
use macros::Receivable;

#[derive(Receivable)]
pub struct PlayerPositionRotation {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool,
}
