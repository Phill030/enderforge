use crate::{
    errors::DecodeError,
    types::{Position, VarInt, VarLong},
    utils::MAX_STRING_LEN,
};
use byteorder::{BigEndian, ReadBytesExt};
use std::io::{Cursor, Read};
use uuid::Uuid;

static SEGMENT_BITS: u8 = 0x7F;
static CONTINUE_BIT: u8 = 0x80;

pub trait Decoder {
    type Output;

    fn decode<R: Read>(reader: &mut R) -> Result<Self::Output, DecodeError>;
}

pub trait DecoderReadExt {
    fn read_bool(&mut self) -> Result<bool, DecodeError>;
    fn read_string(&mut self, max_length: u16) -> Result<String, DecodeError>;
    fn read_byte_array(&mut self) -> Result<Vec<u8>, DecodeError>;
    fn read_var_i32(&mut self) -> Result<i32, DecodeError>;
    fn read_var_i64(&mut self) -> Result<i64, DecodeError>;
}

pub trait ReceiveFromStream: Sized {
    fn receive(buf: &mut Cursor<Vec<u8>>) -> Result<Self, DecodeError>;
}

macro_rules! read_signed_var_int (
    ($type: ident, $name: ident, $max_bytes: expr) => (
        fn $name(&mut self) -> Result<$type, DecodeError> {
            let mut bytes = 0;
            let mut output = 0;

            loop {
                let byte = self.read_u8()?;
                let value = (byte & SEGMENT_BITS) as $type;

                output |= value << 7 * bytes;
                bytes += 1;

                if bytes > $max_bytes {
                    return Err(DecodeError::VarIntTooLong { max_bytes: $max_bytes })
                }

                if (byte & CONTINUE_BIT) == 0 {
                    break;
                }
            }

            Ok(output)
        }
   );
);

impl Decoder for u8 {
    type Output = Self;

    fn decode<R: Read>(reader: &mut R) -> Result<Self::Output, DecodeError> {
        Ok(reader.read_u8()?)
    }
}

impl Decoder for i8 {
    type Output = Self;

    fn decode<R: Read>(reader: &mut R) -> Result<Self::Output, DecodeError> {
        Ok(reader.read_i8()?)
    }
}

impl Decoder for i16 {
    type Output = Self;

    fn decode<R: Read>(reader: &mut R) -> Result<Self::Output, DecodeError> {
        Ok(reader.read_i16::<BigEndian>()?)
    }
}

impl Decoder for i32 {
    type Output = Self;

    fn decode<R: Read>(reader: &mut R) -> Result<Self::Output, DecodeError> {
        Ok(reader.read_i32::<BigEndian>()?)
    }
}

impl Decoder for String {
    type Output = Self;

    fn decode<R: Read>(reader: &mut R) -> Result<Self::Output, DecodeError> {
        reader.read_string(32_768)
    }
}

impl Decoder for bool {
    type Output = Self;

    fn decode<R: Read>(reader: &mut R) -> Result<Self::Output, DecodeError> {
        reader.read_bool()
    }
}

impl Decoder for Vec<u8> {
    type Output = Self;

    fn decode<R: Read>(reader: &mut R) -> Result<Self::Output, DecodeError> {
        reader.read_byte_array()
    }
}

impl Decoder for Vec<String> {
    type Output = Self;

    fn decode<R: Read>(reader: &mut R) -> Result<Self::Output, DecodeError> {
        let len = reader.read_var_i32()?;
        let mut buffer: Vec<String> = vec![];

        for _ in 0..len {
            buffer.push(reader.read_string(MAX_STRING_LEN)?);
        }

        Ok(buffer)
    }
}

impl Decoder for Uuid {
    type Output = Self;

    fn decode<R: Read>(reader: &mut R) -> Result<Self::Output, DecodeError> {
        Ok(Uuid::from_u128(reader.read_u128::<BigEndian>()?))
    }
}

impl Decoder for u16 {
    type Output = Self;

    fn decode<R: Read>(reader: &mut R) -> Result<Self::Output, DecodeError> {
        Ok(reader.read_u16::<BigEndian>()?)
    }
}

impl Decoder for u32 {
    type Output = Self;

    fn decode<R: Read>(reader: &mut R) -> Result<Self::Output, DecodeError> {
        Ok(reader.read_u32::<BigEndian>()?)
    }
}

impl Decoder for i64 {
    type Output = Self;

    fn decode<R: Read>(reader: &mut R) -> Result<Self::Output, DecodeError> {
        Ok(reader.read_i64::<BigEndian>()?)
    }
}

impl Decoder for u64 {
    type Output = Self;

    fn decode<R: Read>(reader: &mut R) -> Result<Self::Output, DecodeError> {
        Ok(reader.read_u64::<BigEndian>()?)
    }
}

impl Decoder for f32 {
    type Output = Self;

    fn decode<R: Read>(reader: &mut R) -> Result<Self::Output, DecodeError> {
        Ok(reader.read_f32::<BigEndian>()?)
    }
}

impl Decoder for f64 {
    type Output = Self;

    fn decode<R: Read>(reader: &mut R) -> Result<Self::Output, DecodeError> {
        Ok(reader.read_f64::<BigEndian>()?)
    }
}

impl Decoder for VarInt {
    type Output = Self;

    fn decode<R: Read>(reader: &mut R) -> Result<Self::Output, DecodeError> {
        Ok(VarInt(reader.read_var_i32()?))
    }
}

impl Decoder for VarLong {
    type Output = Self;

    fn decode<R: Read>(reader: &mut R) -> Result<Self::Output, DecodeError> {
        Ok(VarLong(reader.read_var_i64()?))
    }
}

impl Decoder for Position {
    type Output = Self;

    fn decode<R: Read>(reader: &mut R) -> Result<Self::Output, DecodeError> {
        Ok(Position::from(reader.read_u64::<BigEndian>()?))
    }
}

// impl<T: Decoder> Decoder for Option<T> {
//     type Output = Self;

//     fn decode<R: Read>(reader: &mut R) -> Result<Self::Output, DecodeError> {}
// }

impl<R: Read> DecoderReadExt for R {
    fn read_bool(&mut self) -> Result<bool, DecodeError> {
        match self.read_u8()? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(DecodeError::NonBoolValue),
        }
    }

    fn read_byte_array(&mut self) -> Result<Vec<u8>, DecodeError> {
        let length = self.read_var_i32()?;

        let mut buf = vec![0; length as usize];
        self.read_exact(&mut buf)?;

        Ok(buf)
    }

    fn read_string(&mut self, max_length: u16) -> Result<String, DecodeError> {
        let length = self.read_var_i32()? as usize;

        if length as u16 > max_length {
            return Err(DecodeError::StringTooLong { length, max_length });
        }

        let mut buf = vec![0; length];
        self.read_exact(&mut buf)?;

        Ok(String::from_utf8(buf)?)
    }

    read_signed_var_int!(i32, read_var_i32, 5);
    read_signed_var_int!(i64, read_var_i64, 10);
}
