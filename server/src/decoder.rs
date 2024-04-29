use crate::{
    errors::DecodeError,
    types::{BitSet, Position, VarInt, VarLong},
};
use std::io::Cursor;
use tokio::io::{AsyncRead, AsyncReadExt};
use uuid::Uuid;

static SEGMENT_BITS: u8 = 0x7F;
static CONTINUE_BIT: u8 = 0x80;

pub trait Decoder {
    type Output;

    async fn decode<R: AsyncRead + Unpin>(reader: &mut R) -> Result<Self::Output, DecodeError>;
}

pub trait DecoderReadExt {
    async fn read_bool(&mut self) -> Result<bool, DecodeError>;
    async fn read_string(&mut self, max_length: u16) -> Result<String, DecodeError>;
    async fn read_byte_array(&mut self) -> Result<Vec<u8>, DecodeError>;
    async fn read_var_i32(&mut self) -> Result<i32, DecodeError>;
    async fn read_var_i64(&mut self) -> Result<i64, DecodeError>;
}

pub trait ReceiveFromStream: Sized {
    async fn receive(buf: &mut Cursor<Vec<u8>>) -> Result<Self, DecodeError>;
}

macro_rules! read_signed_var_int (
    ($type: ident, $name: ident, $max_bytes: expr) => (
        async fn $name(&mut self) -> Result<$type, DecodeError> {
            let mut bytes = 0;
            let mut output = 0;

            loop {
                let byte = self.read_u8().await?;
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

    async fn decode<R: AsyncRead + Unpin>(reader: &mut R) -> Result<Self::Output, DecodeError> {
        Ok(reader.read_u8().await?)
    }
}

impl Decoder for i8 {
    type Output = Self;

    async fn decode<R: AsyncRead + Unpin>(reader: &mut R) -> Result<Self::Output, DecodeError> {
        Ok(reader.read_i8().await?)
    }
}

impl Decoder for i16 {
    type Output = Self;

    async fn decode<R: AsyncRead + Unpin>(reader: &mut R) -> Result<Self::Output, DecodeError> {
        Ok(reader.read_i16().await?)
    }
}

impl Decoder for i32 {
    type Output = Self;

    async fn decode<R: AsyncRead + Unpin>(reader: &mut R) -> Result<Self::Output, DecodeError> {
        Ok(reader.read_i32().await?)
    }
}

impl Decoder for String {
    type Output = Self;

    async fn decode<R: AsyncRead + Unpin>(reader: &mut R) -> Result<Self::Output, DecodeError> {
        reader.read_string(32_768).await
    }
}

impl Decoder for bool {
    type Output = Self;

    async fn decode<R: AsyncRead + Unpin>(reader: &mut R) -> Result<Self::Output, DecodeError> {
        reader.read_bool().await
    }
}

// impl Decoder for Vec<u8> {
//     type Output = Self;

//     fn decode<R: AsyncRead + Unpin>(reader: &mut R) -> Result<Self::Output, DecodeError> {
//         reader.read_byte_array()
//     }
// }

// impl Decoder for Vec<String> {
//     type Output = Self;

//     fn decode<R: AsyncRead + Unpin>(reader: &mut R) -> Result<Self::Output, DecodeError> {
//         let len = reader.read_var_i32()?;
//         let mut buffer: Vec<String> = vec![0; len as usize];

//         for _ in 0..len {
//             buffer.push(reader.read_string(MAX_STRING_LEN)?);
//         }

//         Ok(buffer)
//     }
// }

impl Decoder for Uuid {
    type Output = Self;

    async fn decode<R: AsyncRead + Unpin>(reader: &mut R) -> Result<Self::Output, DecodeError> {
        Ok(Uuid::from_u128(reader.read_u128().await?))
    }
}

impl Decoder for u16 {
    type Output = Self;

    async fn decode<R: AsyncRead + Unpin>(reader: &mut R) -> Result<Self::Output, DecodeError> {
        Ok(reader.read_u16().await?)
    }
}

impl Decoder for u32 {
    type Output = Self;

    async fn decode<R: AsyncRead + Unpin>(reader: &mut R) -> Result<Self::Output, DecodeError> {
        Ok(reader.read_u32().await?)
    }
}

impl Decoder for i64 {
    type Output = Self;

    async fn decode<R: AsyncRead + Unpin>(reader: &mut R) -> Result<Self::Output, DecodeError> {
        Ok(reader.read_i64().await?)
    }
}

impl Decoder for u64 {
    type Output = Self;

    async fn decode<R: AsyncRead + Unpin>(reader: &mut R) -> Result<Self::Output, DecodeError> {
        Ok(reader.read_u64().await?)
    }
}

impl Decoder for f32 {
    type Output = Self;

    async fn decode<R: AsyncRead + Unpin>(reader: &mut R) -> Result<Self::Output, DecodeError> {
        Ok(reader.read_f32().await?)
    }
}

impl Decoder for f64 {
    type Output = Self;

    async fn decode<R: AsyncRead + Unpin>(reader: &mut R) -> Result<Self::Output, DecodeError> {
        Ok(reader.read_f64().await?)
    }
}

impl Decoder for VarInt {
    type Output = Self;

    async fn decode<R: AsyncRead + Unpin>(reader: &mut R) -> Result<Self::Output, DecodeError> {
        Ok(VarInt(reader.read_var_i32().await?))
    }
}

impl Decoder for VarLong {
    type Output = Self;

    async fn decode<R: AsyncRead + Unpin>(reader: &mut R) -> Result<Self::Output, DecodeError> {
        Ok(VarLong(reader.read_var_i64().await?))
    }
}

impl Decoder for Position {
    type Output = Self;

    async fn decode<R: AsyncRead + Unpin>(reader: &mut R) -> Result<Self::Output, DecodeError> {
        Ok(Position::from(reader.read_u64().await?))
    }
}

impl<T: Decoder<Output = T>> Decoder for Vec<T> {
    type Output = Self;

    async fn decode<R: AsyncRead + Unpin>(reader: &mut R) -> Result<Self::Output, DecodeError> {
        let len = reader.read_var_i32().await?;
        let mut x_vec: Vec<T> = Vec::with_capacity(len as usize);

        for _ in 0..len {
            x_vec.push(T::decode(reader).await?);
        }

        Ok(x_vec)
    }
}

impl<R: AsyncRead + Unpin> DecoderReadExt for R {
    async fn read_bool(&mut self) -> Result<bool, DecodeError> {
        match self.read_u8().await? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(DecodeError::NonBoolValue),
        }
    }

    async fn read_byte_array(&mut self) -> Result<Vec<u8>, DecodeError> {
        let length = self.read_var_i32().await?;

        let mut buf = vec![0; length as usize];
        self.read_exact(&mut buf).await?;

        Ok(buf)
    }

    async fn read_string(&mut self, max_length: u16) -> Result<String, DecodeError> {
        let length = self.read_var_i32().await? as usize;

        if length as u16 > max_length {
            return Err(DecodeError::StringTooLong { length, max_length });
        }

        let mut buf = vec![0; length];
        self.read_exact(&mut buf).await?;

        Ok(String::from_utf8(buf)?)
    }

    read_signed_var_int!(i32, read_var_i32, 5);
    read_signed_var_int!(i64, read_var_i64, 10);
}
