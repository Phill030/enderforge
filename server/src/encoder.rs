use crate::{
    errors::EncodeError,
    types::{BitSet, Position, VarInt, VarLong},
    utils::MAX_STRING_LEN,
};
use byteorder::{BigEndian, WriteBytesExt};
use nbt::io::Nbt;
use std::{io::Write, net::TcpStream};
use uuid::Uuid;

pub trait Encoder {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError>;
}

/// Trait adds additional helper methods for `Write` to write protocol data.
pub trait EncoderWriteExt {
    fn write_bool(&mut self, value: bool) -> Result<(), EncodeError>;
    fn write_string(&mut self, value: &str, max_length: u16) -> Result<(), EncodeError>;
    fn write_byte_array(&mut self, value: &[u8]) -> Result<(), EncodeError>;
    // fn write_compound_tag(&mut self, value: &CompoundTag) -> Result<(), EncodeError>;
    fn write_var_i32(&mut self, value: VarInt) -> Result<(), EncodeError>;
    fn write_var_i64(&mut self, value: VarLong) -> Result<(), EncodeError>;
    fn write_uuid(&mut self, value: Uuid) -> Result<(), EncodeError>;
}

pub trait SendToStream {
    fn send(&self, stream: &mut TcpStream) -> Result<(), EncodeError>;
}

macro_rules! write_signed_var_int (
    ($type: ident, $name: ident) => (
        fn $name(&mut self, mut value: $type) -> Result<(), EncodeError> {
            loop {
                let mut byte = (value.0 & 0b01111111) as u8;
                value = $type(value.0 >> 7);

                if value.0 != 0 {
                    byte |= 0b10000000;
                }

                self.write_u8(byte)?;

                if value.0 == 0 {
                   break;
                }
            }

            Ok(())
        }
    )
);

impl<W: Write> EncoderWriteExt for W {
    fn write_bool(&mut self, value: bool) -> Result<(), EncodeError> {
        if value {
            self.write_u8(1)?;
        } else {
            self.write_u8(0)?;
        }

        Ok(())
    }

    fn write_string(&mut self, value: &str, max_length: u16) -> Result<(), EncodeError> {
        let length = value.len();

        if length > max_length as usize {
            return Err(EncodeError::StringTooLong { length, max_length });
        }

        self.write_var_i32(value.len().into())?;
        self.write_all(value.as_bytes())?;

        Ok(())
    }

    fn write_byte_array(&mut self, value: &[u8]) -> Result<(), EncodeError> {
        self.write_var_i32(value.len().into())?;
        self.write_all(value)?;

        Ok(())
    }

    fn write_uuid(&mut self, value: Uuid) -> Result<(), EncodeError> {
        Ok(self.write_all(value.as_bytes())?)
    }

    write_signed_var_int!(VarInt, write_var_i32);
    write_signed_var_int!(VarLong, write_var_i64);
}

impl Encoder for u8 {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        Ok(writer.write_u8(*self)?)
    }
}

impl Encoder for i8 {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        Ok(writer.write_i8(*self)?)
    }
}

impl Encoder for i16 {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        Ok(writer.write_i16::<BigEndian>(*self)?)
    }
}

impl Encoder for i32 {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        Ok(writer.write_i32::<BigEndian>(*self)?)
    }
}

impl Encoder for u16 {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        Ok(writer.write_u16::<BigEndian>(*self)?)
    }
}

impl Encoder for u32 {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        Ok(writer.write_u32::<BigEndian>(*self)?)
    }
}

impl Encoder for i64 {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        Ok(writer.write_i64::<BigEndian>(*self)?)
    }
}

impl Encoder for u64 {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        Ok(writer.write_u64::<BigEndian>(*self)?)
    }
}

impl Encoder for f32 {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        Ok(writer.write_f32::<BigEndian>(*self)?)
    }
}

impl Encoder for f64 {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        Ok(writer.write_f64::<BigEndian>(*self)?)
    }
}

impl Encoder for String {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        writer.write_string(self, 32_768)
    }
}

impl Encoder for &str {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        writer.write_string(self, 32_768)
    }
}

impl Encoder for bool {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        writer.write_bool(*self)
    }
}

impl Encoder for Vec<u8> {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        writer.write_byte_array(self)
    }
}

impl Encoder for Vec<String> {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        writer.write_var_i32(VarInt::from(self.len()))?;

        for val in self {
            writer.write_string(val, MAX_STRING_LEN)?;
        }

        Ok(())
    }
}

impl Encoder for Nbt {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        Ok(self.to_networked_writer(writer)?)
    }
}

impl Encoder for VarInt {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        writer.write_var_i32(*self)
    }
}

impl Encoder for VarLong {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        writer.write_var_i64(*self)
    }
}

impl Encoder for Uuid {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        Ok(writer.write_u128::<BigEndian>(self.as_u128())?)
    }
}

impl Encoder for Position {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        Ok(writer.write_u64::<BigEndian>(
            ((self.x as u64 & 0x03FF_FFFF) << 38) | ((self.z as u64 & 0x03FF_FFFF) << 12) | (self.y as u64 & 0xFFF),
        )?)
    }
}

// TODO: Replace u8 with T
// TODO: Decoder for [u8; N]
impl<const N: usize> Encoder for [u8; N]
where
    [u8; N]: Sized,
{
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        writer.write_var_i32(self.len().into())?;
        writer.write_all(self)?;
        Ok(())
    }
}

// TODO: Decoder for BitSet
impl Encoder for BitSet {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        writer.write_var_i32(self.0.len().into())?;

        for val in &self.0 {
            writer.write_i64::<BigEndian>(*val)?;
        }

        Ok(())
    }
}

impl<T: Encoder> Encoder for Option<T> {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        Ok(if let Some(val) = self {
            val.encode(writer)?
        } else {
            0u8.encode(writer)? // u8!!
        })
    }
}

pub mod rest {
    use crate::errors::EncodeError;
    use std::io::Write;

    pub fn encode<W: Write>(value: &[u8], writer: &mut W) -> Result<(), EncodeError> {
        writer.write_all(value)?;

        Ok(())
    }
}
