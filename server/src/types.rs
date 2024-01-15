use std::fmt::Display;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct VarInt(pub i32);

impl From<VarInt> for i32 {
    fn from(value: VarInt) -> Self {
        value.0
    }
}

impl From<usize> for VarInt {
    fn from(value: usize) -> Self {
        Self(value as i32)
    }
}

impl Display for VarInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.0)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct VarLong(pub i64);

impl From<VarLong> for i64 {
    fn from(value: VarLong) -> Self {
        value.0
    }
}

impl From<i64> for VarLong {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Position {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn distance_to(&self, position: &Position) -> f32 {
        let delta_x = (position.x - self.x).powf(2.0);
        let delta_y = (position.y - self.y).powf(2.0);
        let delta_z = (position.z - self.z).powf(2.0);

        (delta_x + delta_y + delta_z).sqrt()
    }

    pub fn add(&mut self, position: &Position) {
        self.x += position.x;
        self.y += position.y;
        self.z += position.z;
    }

    pub fn sub(&mut self, position: &Position) {
        self.x -= position.x;
        self.y -= position.y;
        self.z -= position.z;
    }

    pub fn mul(&mut self, position: &Position) {
        self.x *= position.x;
        self.y *= position.y;
        self.z *= position.z;
    }

    pub fn div(&mut self, position: &Position) {
        self.x /= position.x;
        self.y /= position.y;
        self.z /= position.z;
    }
}

impl From<u64> for Position {
    fn from(value: u64) -> Self {
        let mut x: f32 = (value >> 38) as f32;
        let mut z: f32 = (value << 52 >> 52) as f32;
        let mut y: f32 = (value << 26 >> 38) as f32;

        if x >= (1u64 << 25u64) as f32 {
            x -= (1 << 26) as f32;
        }

        if y >= (1u64 << 11u64) as f32 {
            y -= (1 << 12) as f32;
        }

        if z >= (1u64 << 25u64) as f32 {
            z -= (1 << 26) as f32;
        }

        Self { x, y, z }
    }
}

impl From<Position> for u64 {
    fn from(value: Position) -> Self {
        ((value.x as u64 & 0x03FF_FFFF) << 38) | ((value.z as u64 & 0x03FF_FFFF) << 12) | (value.y as u64 & 0xFFF)
    }
}

/// https://wiki.vg/Protocol#BitSet
pub struct BitSet(pub Vec<i64>);
impl BitSet {
    pub fn new(data: Vec<i64>) -> Self {
        Self(data)
    }

    pub fn empty() -> Self {
        Self(vec![])
    }
}
