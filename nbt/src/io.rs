#![allow(private_bounds)]
use super::types::{NbtReader, NbtWriter, Tag};
use async_recursion::async_recursion;
use flate2::{
    read::{GzDecoder, ZlibDecoder},
    write::{GzEncoder, ZlibEncoder},
    Compression,
};
use std::{
    collections::HashMap,
    fmt,
    io::{self, Result},
};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Nbt {
    title: String,
    content: HashMap<String, Tag>,
}
impl Nbt {
    pub fn new<S>(title: S, content: HashMap<S, Tag>) -> Self
    where
        S: Into<String>,
    {
        Self {
            title: title.into(),
            content: content.into_iter().map(|(k, v)| (k.into(), v)).collect(),
        }
    }

    pub async fn to_writer<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: AsyncWrite + NbtWriter + Unpin + Send,
    {
        writer.write_u8(0x0a).await?;
        writer.write_bare_string(&self.title).await?;

        for (name, nbt) in self.content.iter() {
            writer.write_u8(nbt.id()).await?;
            writer.write_bare_string(name).await?;
            nbt.encode(writer).await?;
        }

        Ok(writer.close_nbt().await?)
    }

    /// Networked NBT's are missing the title of the root `TAG_COMPOUND`
    pub async fn to_networked_writer<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: AsyncWrite + NbtWriter + Unpin + Send,
    {
        writer.write_u8(0x0a).await?;

        for (name, nbt) in self.content.iter() {
            writer.write_u8(nbt.id()).await?;
            writer.write_bare_string(name).await?;
            nbt.encode(writer).await?;
        }

        Ok(writer.close_nbt().await?)
    }

    // pub async fn to_zlib_writer<W: AsyncWrite + NbtWriter>(&mut self, writer: &mut W) -> io::Result<()> {
    //     Ok(self.to_writer(&mut GzEncoder::new(writer, Compression::default())).await?)
    // }

    // pub async fn to_gzip_writer<W: AsyncWrite + NbtWriter>(&self, writer: &mut W) -> io::Result<()> {
    //     Ok(self.to_writer(&mut ZlibEncoder::new(writer, Compression::default())).await?)
    // }

    pub async fn from_reader<R>(reader: &mut R) -> io::Result<Nbt>
    where
        R: AsyncRead + NbtReader + Unpin + Send,
    {
        let (tag, title) = reader.emit_next_header().await?;

        if tag != 0x0a {
            panic!("TODO HANDLE ERROR! Must start with Root compound");
        }

        let content = Tag::decode(tag, reader).await;
        match content {
            Ok(Tag::Compound(map)) => Ok(Nbt { title, content: map }),
            _ => panic!("TODO HANDLE ERROR! NO ROOT COMPONENT"),
        }
    }

    // pub async fn from_zlib_reader<R: AsyncRead + NbtReader>(reader: &mut R) -> io::Result<Nbt> {
    //     let mut zlib = ZlibDecoder::new(reader);
    //     Self::from_reader(&mut zlib)
    // }

    // pub async fn from_gzip_reader<R: AsyncRead + NbtReader>(reader: &mut R) -> io::Result<Nbt> {
    //     let mut zlib = GzDecoder::new(reader);
    //     Self::from_reader(&mut zlib)
    // }
}

impl fmt::Display for Nbt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TAG_Compound(\"{}\"): {} entry(ies)\n{{\n", self.title, self.content.len())?;
        for (name, tag) in self.content.iter() {
            write!(f, "  {}(\"{}\"): ", tag.tag_name(), name)?;
            tag.print(f, 2)?;
            writeln!(f)?;
        }
        write!(f, "}}")
    }
}

impl Tag {
    pub fn id(&self) -> u8 {
        match self {
            Tag::Byte(_) => 0x01,
            Tag::Short(_) => 0x02,
            Tag::Int(_) => 0x03,
            Tag::Long(_) => 0x04,
            Tag::Float(_) => 0x05,
            Tag::Double(_) => 0x06,
            Tag::ByteArray(_) => 0x07,
            Tag::String(_) => 0x08,
            Tag::List(_) => 0x09,
            Tag::Compound(_) => 0x0A,
            Tag::IntArray(_) => 0x0B,
            Tag::LongArray(_) => 0x0C,
        }
    }

    pub fn tag_name(&self) -> &str {
        match *self {
            Tag::Byte(_) => "TAG_Byte",
            Tag::Short(_) => "TAG_Short",
            Tag::Int(_) => "TAG_Int",
            Tag::Long(_) => "TAG_Long",
            Tag::Float(_) => "TAG_Float",
            Tag::Double(_) => "TAG_Double",
            Tag::ByteArray(_) => "TAG_ByteArray",
            Tag::String(_) => "TAG_String",
            Tag::List(_) => "TAG_List",
            Tag::Compound(_) => "TAG_Compound",
            Tag::IntArray(_) => "TAG_IntArray",
            Tag::LongArray(_) => "TAG_LongArray",
        }
    }

    #[async_recursion()]
    async fn encode<W: AsyncWrite + Unpin + Send>(&self, writer: &mut W) -> io::Result<()> {
        Ok(match self {
            Tag::Byte(value) => writer.write_bare_byte(*value).await?,
            Tag::Short(value) => writer.write_bare_short(*value).await?,
            Tag::Int(value) => writer.write_bare_int(*value).await?,
            Tag::Long(value) => writer.write_bare_long(*value).await?,
            Tag::Float(value) => writer.write_bare_float(*value).await?,
            Tag::Double(value) => writer.write_bare_double(*value).await?,
            Tag::String(value) => writer.write_bare_string(value).await?,
            Tag::List(values) => {
                if values.is_empty() {
                    writer.write_u8(0).await?; // TAG_End
                    writer.write_i32(0).await?; // Length of the list
                } else {
                    let first_id = values[0].id();

                    writer.write_u8(first_id).await?;
                    writer.write_i32(values.len() as i32).await?;

                    for nbt in values {
                        if nbt.id() != first_id {
                            todo!("HANDLE PANIC!");
                        }

                        nbt.encode(writer).await?;
                    }
                }
            }
            Tag::Compound(values) => {
                for (name, nbt) in values {
                    writer.write_u8(nbt.id()).await?;
                    writer.write_bare_string(name).await?;
                    nbt.encode(writer).await?;
                }

                writer.close_nbt().await?;
            }
            Tag::ByteArray(values) => writer.write_bare_byte_array(&values[..]).await?,
            Tag::IntArray(values) => writer.write_bare_int_array(&values[..]).await?,
            Tag::LongArray(values) => writer.write_bare_long_array(&values[..]).await?,
        })
    }

    #[async_recursion()]
    async fn decode<R: AsyncRead + Unpin + Send>(id: u8, reader: &mut R) -> Result<Tag> {
        Ok(match id {
            0x01 => Tag::Byte(reader.read_bare_byte().await?),
            0x02 => Tag::Short(reader.read_bare_short().await?),
            0x03 => Tag::Int(reader.read_bare_int().await?),
            0x04 => Tag::Long(reader.read_bare_long().await?),
            0x05 => Tag::Float(reader.read_bare_float().await?),
            0x06 => Tag::Double(reader.read_bare_double().await?),
            0x07 => Tag::ByteArray(reader.read_bare_byte_array().await?),
            0x08 => Tag::String(reader.read_bare_string().await?),
            0x09 => {
                let id = reader.read_u8().await?;
                let len = reader.read_i32().await? as usize;
                let mut buf = Vec::with_capacity(len);
                for _ in 0..len {
                    buf.push(Tag::decode(id, reader).await?);
                }
                Tag::List(buf)
            }
            0x0a => {
                let mut map = HashMap::new();
                loop {
                    let (id, name) = reader.emit_next_header().await?;

                    if id.eq(&0x00) {
                        break;
                    }

                    let tag = Tag::decode(id, reader).await?;
                    map.insert(name, tag);
                }
                Tag::Compound(map)
            }
            0x0b => Tag::IntArray(reader.read_bare_int_array().await?),
            0x0c => Tag::LongArray(reader.read_bare_long_array().await?),
            _ => panic!("TODO HANDLE ERROR! INVALID TYPE ID "),
        })
    }

    pub fn print(&self, f: &mut fmt::Formatter, offset: usize) -> fmt::Result {
        match *self {
            Tag::Byte(v) => write!(f, "{}", v),
            Tag::Short(v) => write!(f, "{}", v),
            Tag::Int(v) => write!(f, "{}", v),
            Tag::Long(v) => write!(f, "{}", v),
            Tag::Float(v) => write!(f, "{}", v),
            Tag::Double(v) => write!(f, "{}", v),
            Tag::ByteArray(ref v) => write!(f, "{:?}", v),
            Tag::String(ref v) => write!(f, "{}", v),
            Tag::IntArray(ref v) => write!(f, "{:?}", v),
            Tag::LongArray(ref v) => write!(f, "{:?}", v),
            Tag::List(ref v) => {
                if v.is_empty() {
                    write!(f, "zero entries")
                } else {
                    write!(
                        f,
                        "{} entries of type {}\n{:>width$}\n",
                        v.len(),
                        v[0].tag_name(),
                        "{",
                        width = offset + 1
                    )?;
                    for tag in v {
                        let new_offset = offset + 2;
                        write!(f, "{:>width$}(None): ", tag.tag_name(), width = new_offset + tag.tag_name().len())?;
                        tag.print(f, new_offset)?;
                        writeln!(f)?;
                    }
                    write!(f, "{:>width$}", "}", width = offset + 1)
                }
            }
            Tag::Compound(ref v) => {
                write!(f, "{} entry(ies)\n{:>width$}\n", v.len(), "{", width = offset + 1)?;
                for (name, tag) in v {
                    let new_offset = offset + 2;
                    write!(
                        f,
                        "{:>width$}({}): ",
                        tag.tag_name(),
                        name,
                        width = new_offset + tag.tag_name().len()
                    )?;
                    tag.print(f, new_offset)?;
                    writeln!(f)?;
                }
                write!(f, "{:>width$}", "}", width = offset + 1)
            }
        }
    }
}
