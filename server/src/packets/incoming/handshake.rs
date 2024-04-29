use crate::{
    decoder::{Decoder, ReceiveFromStream},
    packets::{outgoing::player_list_response::PlayerListResponse, packet::OutgoingPacket},
    types::VarInt,
    GameplayState,
};
use macros::Receivable;
use std::io::Cursor;
use tokio::sync::mpsc::Sender;

#[derive(Receivable)]
pub struct Handshake {
    pub protocol_version: VarInt,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: VarInt,
}

impl Handshake {
    pub async fn new(cursor: &mut Cursor<Vec<u8>>, state: &mut GameplayState, sender: &Sender<OutgoingPacket>) {
        let handshake = Handshake::receive(cursor).await.unwrap();
        println!(
            "[HandShake] ProtocolVersion: {} | Address: {} | Port: {} | NextState: {}",
            handshake.protocol_version.0, handshake.server_address, handshake.server_port, handshake.next_state.0
        );

        match handshake.next_state {
            VarInt(1) => {
                sender
                    .send(OutgoingPacket::PlayerListResponse(PlayerListResponse::new(
                        "1.20.4".to_string(),
                        765,
                        123_456,
                        "https://phill030.de".to_string(),
                    )))
                    .await
                    .unwrap();
                *state = GameplayState::Status;
            }
            VarInt(2) => *state = GameplayState::Login,
            _ => {
                // SHUT DOWN!
            }
        }
    }
}
