use crate::{
    errors::EncodeError,
    types::{BitSet, Position, VarInt, VarLong},
    utils::MAX_STRING_LEN,
};
use nbt::io::Nbt;
use tokio::{
    io::{AsyncWrite, AsyncWriteExt},
    net::TcpStream,
};
use uuid::Uuid;

pub trait Encoder {
    async fn encode<W: AsyncWrite + Unpin + Send>(&self, writer: &mut W) -> Result<(), EncodeError>;
}

/// Trait adds additional helper methods for `AsyncWrite` to write protocol data.
pub trait EncoderWriteExt {
    async fn write_bool(&mut self, value: bool) -> Result<(), EncodeError>;
    async fn write_string(&mut self, value: &str, max_length: u16) -> Result<(), EncodeError>;
    async fn write_byte_array(&mut self, value: &[u8]) -> Result<(), EncodeError>;
    async fn write_var_i32(&mut self, value: VarInt) -> Result<(), EncodeError>;
    async fn write_var_i64(&mut self, value: VarLong) -> Result<(), EncodeError>;
    async fn write_uuid(&mut self, value: Uuid) -> Result<(), EncodeError>;
}

pub trait SendToStream {
    async fn send(&self, stream: &mut TcpStream) -> Result<(), EncodeError>;
}

macro_rules! write_signed_var_int (
    ($type: ident, $name: ident) => (
        async fn $name(&mut self, mut value: $type) -> Result<(), EncodeError> {
            loop {
                let mut byte = (value.0 & 0b01111111) as u8;
                value = $type(value.0 >> 7);

                if value.0 != 0 {
                    byte |= 0b10000000;
                }

                self.write_u8(byte).await?;

                if value.0 == 0 {
                   break;
                }
            }

            Ok(())
        }
    )
);

impl<W: AsyncWrite + Unpin> EncoderWriteExt for W {
    async fn write_bool(&mut self, value: bool) -> Result<(), EncodeError> {
        if value {
            self.write_u8(1).await?;
        } else {
            self.write_u8(0).await?;
        }

        Ok(())
    }

    async fn write_string(&mut self, value: &str, max_length: u16) -> Result<(), EncodeError> {
        let length = value.len();

        if length > max_length as usize {
            return Err(EncodeError::StringTooLong { length, max_length });
        }

        self.write_var_i32(value.len().into()).await?;
        self.write_all(value.as_bytes()).await?;

        Ok(())
    }

    async fn write_byte_array(&mut self, value: &[u8]) -> Result<(), EncodeError> {
        self.write_var_i32(value.len().into()).await?;
        self.write_all(value).await?;

        Ok(())
    }

    async fn write_uuid(&mut self, value: Uuid) -> Result<(), EncodeError> {
        Ok(self.write_all(value.as_bytes()).await?)
    }

    write_signed_var_int!(VarInt, write_var_i32);
    write_signed_var_int!(VarLong, write_var_i64);
}

impl Encoder for u8 {
    async fn encode<W: AsyncWrite + Unpin + Send>(&self, writer: &mut W) -> Result<(), EncodeError> {
        Ok(writer.write_u8(*self).await?)
    }
}

impl Encoder for i8 {
    async fn encode<W: AsyncWrite + Unpin + Send>(&self, writer: &mut W) -> Result<(), EncodeError> {
        Ok(writer.write_i8(*self).await?)
    }
}

impl Encoder for i16 {
    async fn encode<W: AsyncWrite + Unpin + Send>(&self, writer: &mut W) -> Result<(), EncodeError> {
        Ok(writer.write_i16(*self).await?)
    }
}

impl Encoder for i32 {
    async fn encode<W: AsyncWrite + Unpin + Send>(&self, writer: &mut W) -> Result<(), EncodeError> {
        Ok(writer.write_i32(*self).await?)
    }
}

impl Encoder for u16 {
    async fn encode<W: AsyncWrite + Unpin + Send>(&self, writer: &mut W) -> Result<(), EncodeError> {
        Ok(writer.write_u16(*self).await?)
    }
}

impl Encoder for u32 {
    async fn encode<W: AsyncWrite + Unpin + Send>(&self, writer: &mut W) -> Result<(), EncodeError> {
        Ok(writer.write_u32(*self).await?)
    }
}

impl Encoder for i64 {
    async fn encode<W: AsyncWrite + Unpin + Send>(&self, writer: &mut W) -> Result<(), EncodeError> {
        Ok(writer.write_i64(*self).await?)
    }
}

impl Encoder for u64 {
    async fn encode<W: AsyncWrite + Unpin + Send>(&self, writer: &mut W) -> Result<(), EncodeError> {
        Ok(writer.write_u64(*self).await?)
    }
}

impl Encoder for f32 {
    async fn encode<W: AsyncWrite + Unpin + Send>(&self, writer: &mut W) -> Result<(), EncodeError> {
        Ok(writer.write_f32(*self).await?)
    }
}

impl Encoder for f64 {
    async fn encode<W: AsyncWrite + Unpin + Send>(&self, writer: &mut W) -> Result<(), EncodeError> {
        Ok(writer.write_f64(*self).await?)
    }
}

impl Encoder for String {
    async fn encode<W: AsyncWrite + Unpin + Send>(&self, writer: &mut W) -> Result<(), EncodeError> {
        Ok(writer.write_string(self, 32_768).await?)
    }
}

impl Encoder for &str {
    async fn encode<W: AsyncWrite + Unpin + Send>(&self, writer: &mut W) -> Result<(), EncodeError> {
        Ok(writer.write_string(self, 32_768).await?)
    }
}

impl Encoder for bool {
    async fn encode<W: AsyncWrite + Unpin + Send>(&self, writer: &mut W) -> Result<(), EncodeError> {
        Ok(writer.write_bool(*self).await?)
    }
}

impl Encoder for Vec<u8> {
    async fn encode<W: AsyncWrite + Unpin + Send>(&self, writer: &mut W) -> Result<(), EncodeError> {
        Ok(writer.write_byte_array(self).await?)
    }
}

impl Encoder for Vec<String> {
    async fn encode<W: AsyncWrite + Unpin + Send>(&self, writer: &mut W) -> Result<(), EncodeError> {
        writer.write_var_i32(VarInt::from(self.len())).await?;

        for val in self {
            writer.write_string(val, MAX_STRING_LEN).await?;
        }

        Ok(())
    }
}

impl Encoder for Nbt {
    async fn encode<W: AsyncWrite + Unpin + Send>(&self, writer: &mut W) -> Result<(), EncodeError> {
        Ok(self.to_networked_writer(writer).await?)
    }
}

impl Encoder for VarInt {
    async fn encode<W: AsyncWrite + Unpin + Send>(&self, writer: &mut W) -> Result<(), EncodeError> {
        Ok(writer.write_var_i32(*self).await?)
    }
}

impl Encoder for VarLong {
    async fn encode<W: AsyncWrite + Unpin + Send>(&self, writer: &mut W) -> Result<(), EncodeError> {
        Ok(writer.write_var_i64(*self).await?)
    }
}

impl Encoder for Uuid {
    async fn encode<W: AsyncWrite + Unpin + Send>(&self, writer: &mut W) -> Result<(), EncodeError> {
        Ok(writer.write_u128(self.as_u128()).await?)
    }
}

impl Encoder for Position {
    async fn encode<W: AsyncWrite + Unpin + Send>(&self, writer: &mut W) -> Result<(), EncodeError> {
        let pos = ((self.x as u64 & 0x03FF_FFFF) << 38) | ((self.z as u64 & 0x03FF_FFFF) << 12) | (self.y as u64 & 0xFFF);
        Ok(writer.write_u64(pos).await?)
    }
}

impl<const N: usize> Encoder for [u8; N]
where
    [u8; N]: Sized,
{
    async fn encode<W: AsyncWrite + Unpin + Send>(&self, writer: &mut W) -> Result<(), EncodeError> {
        writer.write_var_i32(self.len().into()).await?;
        writer.write_all(self).await?;

        Ok(())
    }
}

// TODO: Decoder for BitSet
impl Encoder for BitSet {
    async fn encode<W: AsyncWrite + Unpin + Send>(&self, writer: &mut W) -> Result<(), EncodeError> {
        writer.write_var_i32(self.0.len().into()).await?;

        for val in &self.0 {
            writer.write_i64(*val).await?;
        }

        Ok(())
    }
}

impl<T: Encoder> Encoder for Option<T> {
    async fn encode<W: AsyncWrite + Unpin + Send>(&self, writer: &mut W) -> Result<(), EncodeError> {
        Ok(if let Some(val) = self {
            val.encode(writer).await
        } else {
            0u8.encode(writer).await // u8!!
        }?)
    }
}
