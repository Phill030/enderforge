use crate::encoder::EncoderWriteExt;
use crate::{
    decoder::{Decoder, ReceiveFromStream},
    encoder::Encoder,
    errors::EncodeError,
    packets::packet::OutgoingPacket,
    types::VarInt,
    GameplayState,
};
use macros::{Receivable, Serializable, Streamable};
use std::{io::Cursor, ops::Add};
use tokio::sync::mpsc::Sender;
use uuid::Uuid;

#[derive(Receivable)]
struct LoginStart {
    pub username: String,
    pub uuid: Uuid,
}

#[derive(Streamable)]
#[packet_id(0x02)]
pub struct LoginSuccess {
    uuid: Uuid,
    username: String,
    property: Vec<Property>,
}

#[derive(Serializable, Clone)]
struct Property {
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
    pub async fn new(cursor: &mut Cursor<Vec<u8>>, state: &mut GameplayState, sender: &Sender<OutgoingPacket>) {
        let login_start = LoginStart::receive(cursor).await.unwrap();
        println!("[Login] Username: {} | UUID: {}", login_start.username, login_start.uuid);

        let uuid = Uuid::new_v3(
            &Uuid::NAMESPACE_URL,
            String::from("OfflinePlayer:").add(&login_start.username).as_bytes(),
        );

        sender
            .send(OutgoingPacket::LoginSuccess(LoginSuccess::new(uuid, login_start.username.clone())))
            .await
            .unwrap();

        *state = GameplayState::LoginAcknowledge;
    }
}
