use super::outgoing::{
    chunk::{ChunkDataUpdateLight, SetDefaultSpawnPosition, SynchronizePlayerPosition},
    finish_configuration::FinishConfiguration,
    game_event::GameEvent,
    keep_alive::KeepAlive,
    login::LoginSuccess,
    play_login::PlayLogin,
    player_disconnect::PlayDisconnect,
    player_list_response::PlayerListResponse,
    registry_data::RegistryData,
    status::Status,
};
use crate::errors::EncodeError;

pub enum OutgoingPacket {
    KeepAlive(KeepAlive),
    PlayerListResponse(PlayerListResponse),
    Status(Status),
    LoginSuccess(LoginSuccess),
    RegistryData(RegistryData),
    FinishConfiguration(FinishConfiguration),
    PlayLogin(PlayLogin),
    ChunkDataUpdateLight(ChunkDataUpdateLight),
    SynchronizePlayerPosition(SynchronizePlayerPosition),
    GameEvent(GameEvent),
    SetDefaultSpawnPosition(SetDefaultSpawnPosition),
    PlayDisconnect(PlayDisconnect),
}

pub trait Packet {
    async fn to_bytes(&self) -> Result<Vec<u8>, EncodeError>;
}
