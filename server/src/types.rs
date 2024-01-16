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

/// https://wiki.vg/Protocol#BitSet
pub struct BitSet(pub Vec<i64>);
impl BitSet {
    /// Constructor to create a new BitSet with a specified size
    ///
    /// # Arguments
    ///
    /// * `size` - The number of bits in the BitSet
    ///
    /// # Returns
    ///
    /// A new BitSet with all bits initially set to 0
    pub fn new(size: usize) -> Self {
        let num_i64s = (size + 63) / 64;
        let bits = vec![0; num_i64s];
        Self(bits)
    }

    /// Creates an empty BitSet.
    ///
    /// # Returns
    ///
    /// A new BitSet instance with no initial data.
    pub fn empty() -> Self {
        Self(vec![])
    }

    /// Set the bit at the specified index to 1
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the bit to set
    pub fn set(&mut self, index: usize) {
        let i64_index = index / 64;
        let bit_index = index % 64;
        self.0[i64_index] |= 1 << bit_index;
    }

    /// Clear the bit at the specified index to 0
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the bit to clear
    pub fn clear(&mut self, index: usize) {
        let i64_index = index / 64;
        let bit_index = index % 64;
        self.0[i64_index] &= !(1 << bit_index);
    }

    /// Get the value of the bit at the specified index
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the bit to retrieve
    ///
    /// # Returns
    ///
    /// The value of the bit at the specified index (true if set, false if not set)
    pub fn get(&self, index: usize) -> bool {
        let i64_index = index / 64;
        let bit_index = index % 64;
        (self.0[i64_index] & (1 << bit_index)) != 0
    }

    /// Flip the bit at the specified index (change 1 to 0 and vice versa)
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the bit to flip
    pub fn flip(&mut self, index: usize) {
        let i64_index = index / 64;
        let bit_index = index % 64;
        self.0[i64_index] ^= 1 << bit_index;
    }

    /// Get the size of the BitSet
    ///
    /// # Returns
    /// The size of the BitSet
    pub fn size(&self) -> usize {
        self.0.len()
    }
}
