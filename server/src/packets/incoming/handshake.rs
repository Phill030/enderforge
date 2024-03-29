use crate::decoder::{Decoder, ReceiveFromStream};
use crate::encoder::SendToWriter;
use crate::packets::status::PlayerListResponse;
use crate::tcp::server::GameplayState;
use crate::types::VarInt;
use macros::Receivable;
use std::io::Cursor;
use tokio::io::AsyncWriteExt;
use tokio::net::tcp::OwnedWriteHalf;

#[derive(Receivable)]
pub struct HandShake {
    pub protocol_version: VarInt,
    pub server_address: String, // <--- max_len 255
    pub server_port: u16,
    pub next_state: VarInt,
}

impl HandShake {
    pub async fn handle(cursor: &mut Cursor<Vec<u8>>, state: &mut GameplayState, stream: &mut OwnedWriteHalf) {
        let handshake = HandShake::receive(cursor).await.unwrap();

        println!(
            "[HandShake] ProtocolVersion: {} | Address: {} | Port: {} | NextState: {}",
            handshake.protocol_version.0, handshake.server_address, handshake.server_port, handshake.next_state.0
        );

        match handshake.next_state {
            VarInt(1) => {
                PlayerListResponse::new(
                    String::from("1.20.4"),
                    765,
                    123_456,
                    String::from("https://www.youtube.com/watch?v=8gGQFRk5hJw"),
                )
                .send(stream)
                .await
                .unwrap();

                *state = GameplayState::Status;
            }
            VarInt(2) => *state = GameplayState::Login,
            _ => stream.shutdown().await.unwrap(),
        };
    }
}
