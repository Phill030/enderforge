use crate::{
    decoder::{Decoder, ReceiveFromStream},
    encoder::{Encoder, EncoderWriteExt, SendToStream},
    errors::EncodeError,
    packets::config::{FinishConfiguration, RegistryData},
    player::mc_player::McPlayer,
    tcp::server::GameplayState,
    types::VarInt,
};
use macros::{Receivable, Serializable, Streamable};
use std::{
    io::Cursor,
    ops::Add,
    sync::{Arc, Mutex},
};
use tokio::io::AsyncWriteExt;
use tokio::{io::AsyncWrite, net::tcp::OwnedWriteHalf};
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
    pub async fn handle(stream: &mut OwnedWriteHalf, gameplay_state: &mut GameplayState) {
        println!("[LoginAck] Received");
        *gameplay_state = GameplayState::Play;

        RegistryData::create().await.send(stream).await.unwrap();
        FinishConfiguration::default().send(stream).await.unwrap();
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
    pub async fn handle(
        cursor: &mut Cursor<Vec<u8>>,
        players: Arc<Mutex<Vec<McPlayer>>>,
        state: &mut GameplayState,
        stream: &mut OwnedWriteHalf,
    ) {
        let login_start = LoginStart::receive(cursor).await.unwrap();
        println!("[Login] Username: {} | UUID: {}", login_start.username, login_start.uuid);

        let uuid = Uuid::new_v3(
            &Uuid::NAMESPACE_URL,
            String::from("OfflinePlayer:").add(&login_start.username).as_bytes(),
        );
        LoginSuccess::new(uuid, login_start.username.clone()).send(stream).await.unwrap();

        if let Ok(mut players) = players.clone().lock() {
            // TODO: There are 2 different KeepAlives! 1x for Configuration and 1x while Playing, and KeepAlive should only be sent for authorized users (valid uuid)
            // players.push(McPlayer {
            //     stream: stream.try_clone().unwrap(),
            //     username: login_start.username,
            //     uuid,
            // });
        }

        *state = GameplayState::LoginAcknowledge;
    }
}
