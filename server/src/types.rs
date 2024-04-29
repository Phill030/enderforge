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
    fn get(&self, bit_index: u32) -> bool {
        if bit_index < self.len.0 as u32 * 64 {
            let idx = bit_index as usize / 64;
            let bit_pos = bit_index as usize % 64;
            (self.data[idx] & (1 << bit_pos)) != 0
        } else {
            false
        }
    }
}

pub struct PalettedContainer {
    /// Determines how many bits are used to encode entries. Note that not all numbers are valid here.
    bits_per_entry: u8,
    palette: VarInt,
    /// Number of longs in the following array. This value isn't entirely respected by the Notchian client. If it is smaller than expected, it will be overridden by the correct size calculated from Bits Per Entry. If too large, the client will read the specified number of longs, but silently discard all of them afterwards, resulting in a chunk filled with palette entry 0 (which appears to have been unintentional). Present but equal to 0 when Bits Per Entry is 0.
    data_len: VarInt,
    data: Vec<i64>,
}

pub struct ChunkSection {
    /// Number of non-air blocks present in the chunk section. "Non-air" is defined as any fluid and block other than air, cave air, and void air. The client will keep count of the blocks as they are broken and placed, and, if the block count reaches 0, the whole chunk section is not rendered, even if it still has blocks.
    block_count: i16,
    /// Consists of 4096 entries, representing all the blocks in the chunk section.
    block_states: PalettedContainer,
    /// Consists of 64 entries, representing 4×4×4 biome regions in the chunk section.
    biomes: PalettedContainer,
}
