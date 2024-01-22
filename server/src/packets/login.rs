use crate::decoder::{Decoder, ReceiveFromStream};
use crate::encoder::EncoderWriteExt;
use crate::encoder::{Encoder, SendToStream};
use crate::errors::EncodeError;
use crate::packets::chunk::{ChunkDataUpdateLight, SynchronizePlayerPosition};
use crate::packets::config::{FinishConfiguration, RegistryData};
use crate::packets::event::GameEvent;
use crate::packets::play::PlayLogin;
use crate::tcp::server::GameplayState;
use crate::types::VarInt;
use macros::{Receivable, Serializable, Streamable};
use std::ops::Add;
use std::{
    io::{Cursor, Write},
    net::TcpStream,
};
use uuid::Uuid;

#[allow(dead_code)]
#[derive(Receivable)]
pub struct LoginStart {
    pub username: String,
    pub uuid: Uuid,
}

#[derive(Receivable)]
pub struct LoginAcknowledge {}

impl LoginAcknowledge {
    pub fn handle(state: &mut GameplayState, stream: &mut TcpStream) {
        println!("[LoginAck] Received");

        // TODO Refactor to make this inside the Play GameplayState ?
        RegistryData::default().send(stream).unwrap();
        FinishConfiguration::default().send(stream).unwrap();

        PlayLogin::default().send(stream).unwrap();
        ChunkDataUpdateLight::default().send(stream).unwrap();
        SynchronizePlayerPosition::default().send(stream).unwrap();
        GameEvent::default().send(stream).unwrap();
        // SetDefaultSpawnPosition::default().send(stream).unwrap();
        *state = GameplayState::Play;
    }
}

#[derive(Streamable)]
#[packet_id(0x02)]
struct LoginSuccess {
    pub uuid: Uuid,
    pub username: String,
    pub property: Vec<Property>,
}

#[derive(Serializable, Clone)]
pub struct Property {
    pub username: String,
    pub value: String,
    pub is_signed: bool,
    pub signature: Option<String>,
}

impl LoginSuccess {
    pub fn new(uuid: Uuid, username: String) -> Self {
        Self {
            uuid,
            username,
            property: vec![],
        }
    }
}

pub struct Login;
impl Login {
    pub fn handle(cursor: &mut Cursor<Vec<u8>>, state: &mut GameplayState, stream: &mut TcpStream) {
        let login_start = LoginStart::receive(cursor).unwrap();
        println!("[Login] Username: {} | UUID: {}", login_start.username, login_start.uuid);

        let uuid_val = String::from("OfflinePlayer:").add(&login_start.username);
        LoginSuccess::new(Uuid::new_v3(&Uuid::NAMESPACE_URL, uuid_val.as_bytes()), login_start.username)
            .send(stream)
            .unwrap();

        *state = GameplayState::LoginAcknowledge;
    }
}
