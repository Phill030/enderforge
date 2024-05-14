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
        let mut z: f32 = (value << 26 >> 38) as f32;
        let mut y: f32 = (value << 52 >> 52) as f32;

        if x >= (1u64 << 25u64) as f32 {
            x -= (1 << 26) as f32;
        }

        if z >= (1u64 << 25u64) as f32 {
            z -= (1 << 26) as f32;
        }

        if y >= (1u64 << 11u64) as f32 {
            y -= (1 << 12) as f32;
        }

        Self { x, z, y }
    }
}

impl From<Position> for u64 {
    fn from(value: Position) -> Self {
        ((value.x as u64 & 0x03FF_FFFF) << 38) | ((value.z as u64 & 0x03FF_FFFF) << 12) | (value.y as u64 & 0xFFF)
    }
}

pub struct BitSet {
    pub len: VarInt,
    pub data: Vec<i64>,
}

impl BitSet {
    // Create a new BitSet with a specific number of bits (rounded up to nearest long)
    pub fn new(num_bits: i32) -> Self {
        let len = VarInt((num_bits as f64 / 64.0).ceil() as i32); // Calculate number of longs needed
        let data = vec![0; len.0 as usize]; // Initialize data with zeros
        BitSet { len, data }
    }

    // Set a specific bit to 1
    pub fn set(&mut self, bit_index: u32) {
        if bit_index < self.len.0 as u32 * 64 {
            let idx = bit_index as usize / 64;
            let bit_pos = bit_index as usize % 64;
            self.data[idx] |= 1 << bit_pos;
        }
    }

    // Clear a specific bit (set to 0)
    pub fn clear(&mut self, bit_index: u32) {
        if bit_index < self.len.0 as u32 * 64 {
            let idx = bit_index as usize / 64;
            let bit_pos = bit_index as usize % 64;
            self.data[idx] &= !(1 << bit_pos);
        }
    }

    // Check if a specific bit is set
    pub fn get(&self, bit_index: u32) -> bool {
        if bit_index < self.len.0 as u32 * 64 {
            let idx = bit_index as usize / 64;
            let bit_pos = bit_index as usize % 64;
            (self.data[idx] & (1 << bit_pos)) != 0
        } else {
            false
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u16)]
pub enum Block {
    Air = 0,
    Stone = 1,
    Granite = 2,
    PolishedGranite = 3,
    Diorite = 4,
    PolishedDiorite = 5,
    Andesite = 6,
    PolishedAndesite = 7,
    VoidAir = 12817,
    CaveAir = 12818,
}

impl Block {
    pub fn is_empty(&self) -> bool {
        use Block::*;
        matches!(self, Air | VoidAir | CaveAir)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ChunkSection {
    pub blocks: Vec<Block>,
}

impl ChunkSection {
    pub fn get_block(&self, x: i32, y: i32, z: i32) -> Option<&Block> {
        self.blocks.get(((x & 15) + (z & 15) * 16 + (y & 15) * 256) as usize)
    }

    pub fn is_empty(&self, x: i32, y: i32, z: i32) -> bool {
        self.get_block(x, y, z).unwrap_or(&Block::Air).is_empty()
    }

    pub fn max_height_at(&self, x: i32, z: i32) -> Option<i32> {
        for y in (0..16).rev() {
            if let Some(block) = self.get_block(x, y, z) {
                if !block.is_empty() {
                    return Some(y);
                }
            }
        }
        None
    }

    pub fn get_highest_block_at(&self, x: i32, z: i32) -> Option<&Block> {
        for y in (0..16).rev() {
            if let Some(block) = self.get_block(x, y, z) {
                return Some(block);
            }
        }
        None
    }
}
