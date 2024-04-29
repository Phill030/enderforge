use crate::encoder::Encoder;
use macros::Streamable;
use nbt::io::Nbt;
use std::io::Cursor;
use tokio::{fs::File, io::AsyncReadExt};

#[derive(Streamable, Clone)]
#[packet_id(0x05)]
pub struct RegistryData {
    registry_codec: Nbt,
}

impl RegistryData {
    pub async fn create() -> Self {
        let mut f = File::open(r"./dimension_codec.nbt").await.unwrap();
        let mut buffer = vec![];
        f.read_to_end(&mut buffer).await.unwrap();
        let mut cursor = Cursor::new(buffer);
        let nbt = Nbt::from_reader(&mut cursor).await.unwrap();

        Self { registry_codec: nbt }
    }
}
