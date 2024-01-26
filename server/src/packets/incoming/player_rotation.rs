use crate::decoder::Decoder;
use macros::Receivable;

#[derive(Receivable)]
pub struct PlayerRotation {
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool,
}
