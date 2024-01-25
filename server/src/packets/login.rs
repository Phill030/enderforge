use crate::decoder::{Decoder, ReceiveFromStream};
use crate::encoder::EncoderWriteExt;
use crate::encoder::{Encoder, SendToStream};
use crate::errors::EncodeError;
use crate::packets::config::{FinishConfiguration, RegistryData};
use crate::player::mc_player::McPlayer;
use crate::tcp::server::GameplayState;
use crate::types::VarInt;
use macros::{Receivable, Serializable, Streamable};
use std::ops::Add;
use std::sync::{Arc, Mutex};
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
    pub fn handle(stream: &mut TcpStream, gameplay_state: &mut GameplayState) {
        println!("[LoginAck] Received");
        *gameplay_state = GameplayState::Play;

        RegistryData::default().send(stream).unwrap();
        FinishConfiguration::default().send(stream).unwrap();
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
    pub fn handle(cursor: &mut Cursor<Vec<u8>>, players: Arc<Mutex<Vec<McPlayer>>>, state: &mut GameplayState, stream: &mut TcpStream) {
        let login_start = LoginStart::receive(cursor).unwrap();
        println!("[Login] Username: {} | UUID: {}", login_start.username, login_start.uuid);

        let uuid = Uuid::new_v3(
            &Uuid::NAMESPACE_URL,
            String::from("OfflinePlayer:").add(&login_start.username).as_bytes(),
        );
        LoginSuccess::new(uuid, login_start.username.clone()).send(stream).unwrap();

        if let Ok(mut players) = players.clone().lock() {
            // TODO: There are 2 different KeepAlives! 1x for Configuration and 1x while Playing, and KeepAlive should only be sent for authorized users (valid uuid)
            players.push(McPlayer {
                stream: stream.try_clone().unwrap(),
                username: login_start.username,
                uuid,
            });
        }

        *state = GameplayState::LoginAcknowledge;
    }
}
