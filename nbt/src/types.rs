use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::{
    collections::HashMap,
    io::{Read, Result, Write},
};

#[derive(Debug, Clone, PartialEq)]
pub enum Tag {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    String(String),
    List(Vec<Tag>),
    Compound(HashMap<String, Tag>),
    ByteArray(Vec<i8>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}
pub(super) trait NbtWriter {
    fn close_nbt(&mut self) -> Result<()>;
    fn write_bare_string(&mut self, value: &str) -> Result<()>;
    fn write_bare_byte(&mut self, value: i8) -> Result<()>;
    fn write_bare_short(&mut self, value: i16) -> Result<()>;
    fn write_bare_int(&mut self, value: i32) -> Result<()>;
    fn write_bare_long(&mut self, value: i64) -> Result<()>;
    fn write_bare_float(&mut self, value: f32) -> Result<()>;
    fn write_bare_double(&mut self, value: f64) -> Result<()>;
    fn write_bare_int_array(&mut self, value: &[i32]) -> Result<()>;
    fn write_bare_long_array(&mut self, value: &[i64]) -> Result<()>;
    fn write_bare_byte_array(&mut self, value: &[i8]) -> Result<()>;
}

impl<W: Write> NbtWriter for W {
    #[inline]
    fn close_nbt(&mut self) -> Result<()> {
        Ok(self.write_u8(0x00)?)
    }

    #[inline]
    fn write_bare_string(&mut self, value: &str) -> Result<()> {
        let mod_utf8 = cesu8::to_java_cesu8(value);
        self.write_u16::<BigEndian>(mod_utf8.len() as u16)?;
        Ok(self.write_all(&mod_utf8)?)
    }

    #[inline]
    fn write_bare_byte(&mut self, value: i8) -> Result<()> {
        Ok(self.write_i8(value)?)
    }

    #[inline]
    fn write_bare_short(&mut self, value: i16) -> Result<()> {
        Ok(self.write_i16::<BigEndian>(value)?)
    }

    #[inline]
    fn write_bare_int(&mut self, value: i32) -> Result<()> {
        Ok(self.write_i32::<BigEndian>(value)?)
    }

    #[inline]
    fn write_bare_long(&mut self, value: i64) -> Result<()> {
        Ok(self.write_i64::<BigEndian>(value)?)
    }

    #[inline]
    fn write_bare_float(&mut self, value: f32) -> Result<()> {
        Ok(self.write_f32::<BigEndian>(value)?)
    }

    #[inline]
    fn write_bare_double(&mut self, value: f64) -> Result<()> {
        Ok(self.write_f64::<BigEndian>(value)?)
    }

    #[inline]
    fn write_bare_int_array(&mut self, value: &[i32]) -> Result<()> {
        self.write_i32::<BigEndian>(value.len() as i32)?;
        for &v in value {
            self.write_i32::<BigEndian>(v)?;
        }
        Ok(())
    }

    #[inline]
    fn write_bare_long_array(&mut self, value: &[i64]) -> Result<()> {
        self.write_i32::<BigEndian>(value.len() as i32)?;
        for &v in value {
            self.write_i64::<BigEndian>(v)?;
        }
        Ok(())
    }

    #[inline]
    fn write_bare_byte_array(&mut self, value: &[i8]) -> Result<()> {
        self.write_i32::<BigEndian>(value.len() as i32)?;
        for &v in value {
            self.write_i8(v)?;
        }
        Ok(())
    }
}

pub(super) trait NbtReader {
    fn emit_next_header(&mut self) -> Result<(u8, String)>;
    fn read_bare_string(&mut self) -> Result<String>;
    fn read_bare_byte(&mut self) -> Result<i8>;
    fn read_bare_short(&mut self) -> Result<i16>;
    fn read_bare_int(&mut self) -> Result<i32>;
    fn read_bare_long(&mut self) -> Result<i64>;
    fn read_bare_float(&mut self) -> Result<f32>;
    fn read_bare_double(&mut self) -> Result<f64>;
    fn read_bare_int_array(&mut self) -> Result<Vec<i32>>;
    fn read_bare_long_array(&mut self) -> Result<Vec<i64>>;
    fn read_bare_byte_array(&mut self) -> Result<Vec<i8>>;
}

impl<R: Read> NbtReader for R {
    fn emit_next_header(&mut self) -> Result<(u8, String)> {
        let tag = self.read_u8()?;

        match tag {
            0x00 => Ok((tag, "".to_string())),
            _ => {
                let name = self.read_bare_string()?;
                Ok((tag, name))
            }
        }
    }

    #[inline]
    fn read_bare_byte(&mut self) -> Result<i8> {
        Ok(self.read_i8()?)
    }

    #[inline]
    fn read_bare_short(&mut self) -> Result<i16> {
        Ok(self.read_i16::<BigEndian>()?)
    }

    #[inline]
    fn read_bare_int(&mut self) -> Result<i32> {
        Ok(self.read_i32::<BigEndian>()?)
    }

    #[inline]
    fn read_bare_long(&mut self) -> Result<i64> {
        Ok(self.read_i64::<BigEndian>()?)
    }

    #[inline]
    fn read_bare_float(&mut self) -> Result<f32> {
        Ok(self.read_f32::<BigEndian>()?)
    }

    #[inline]
    fn read_bare_double(&mut self) -> Result<f64> {
        Ok(self.read_f64::<BigEndian>()?)
    }

    #[inline]
    fn read_bare_int_array(&mut self) -> Result<Vec<i32>> {
        let len = self.read_i32::<BigEndian>()? as usize;
        let mut buf = Vec::with_capacity(len);

        for _ in 0..len {
            buf.push(self.read_i32::<BigEndian>()?);
        }

        Ok(buf)
    }

    #[inline]
    fn read_bare_long_array(&mut self) -> Result<Vec<i64>> {
        let len = self.read_i32::<BigEndian>()? as usize;

        let mut buf = Vec::with_capacity(len);
        for _ in 0..len {
            buf.push(self.read_i64::<BigEndian>()?);
        }

        Ok(buf)
    }

    #[inline]
    fn read_bare_byte_array(&mut self) -> Result<Vec<i8>> {
        let len = self.read_i32::<BigEndian>()? as usize;
        let mut buf = Vec::with_capacity(len);

        for _ in 0..len {
            buf.push(self.read_i8()?);
        }

        Ok(buf)
    }

    #[inline]
    fn read_bare_string(&mut self) -> Result<String> {
        let len = self.read_u16::<BigEndian>()? as usize;

        if len == 0 {
            return Ok("".to_string());
        }

        let mut bytes = vec![0; len];
        self.read_exact(&mut bytes)?;

        let java_decoded = match cesu8::from_java_cesu8(&bytes) {
            Ok(string) => string,
            Err(_) => panic!("InvalidCesu8String"),
        };

        Ok(if let Ok(string) = std::str::from_utf8(java_decoded.as_bytes()) {
            string.into()
        } else {
            let lossy_string = String::from_utf8_lossy(java_decoded.as_bytes()).into_owned();
            println!("Error decoding utf8 (bytes: {bytes:?}, lossy: \"{lossy_string})\"");
            lossy_string
        })
    }
}
