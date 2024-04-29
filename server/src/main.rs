use packets::packet::{OutgoingPacket, Packet};
use player::Player;
use std::{
    io::{self, Cursor},
    sync::{Arc, Mutex},
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{tcp::OwnedReadHalf, TcpListener, ToSocketAddrs},
    sync::mpsc::{channel, Sender},
    task::JoinSet,
    time,
};

use crate::{
    decoder::{DecoderReadExt, ReceiveFromStream},
    packets::{
        incoming::{
            client_information::ClientInformation, finish_configuration::FinishConfiguration, handshake::Handshake,
            plugin_message::PluginMessage,
        },
        outgoing::{
            chunk::{ChunkDataUpdateLight, SetDefaultSpawnPosition, SynchronizePlayerPosition},
            game_event::GameEvent,
            keep_alive::KeepAlive,
            login::Login,
            login_acknowledge::LoginAcknowledge,
            play_login::PlayLogin,
            status::Status,
        },
    },
};

pub mod decoder;
pub mod encoder;
pub mod errors;
pub mod packets;
pub mod player;
pub mod types;
pub mod utils;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum GameplayState {
    None = -1,
    Status = 1,
    Login = 2,
    LoginAcknowledge = 3,
    Play = 4,
}

pub enum IngameState {
    Config,
    Playing,
}

#[tokio::main]
async fn main() {
    Server::new("0.0.0.0:25565").start().await.unwrap();
}

pub struct Server<E> {
    pub players: Arc<Mutex<Vec<Player>>>,
    pub addr: E,
}

impl<E> Server<E>
where
    E: ToSocketAddrs,
{
    /// Creates a new instance of a server with a given endpoint
    pub fn new(addr: E) -> Self {
        Self {
            players: Arc::new(Mutex::new(vec![])),
            addr,
        }
    }

    /// Starts the newly created instance
    pub async fn start(&self) -> io::Result<()> {
        let listener = TcpListener::bind(&self.addr).await?;

        loop {
            // let players = self.players.clone();
            let (socket, _) = listener.accept().await?;
            let (reader, mut writer) = socket.into_split();
            let (sender, mut receiver) = channel::<OutgoingPacket>(1024);
            let keep_alive_sender = sender.clone();

            let mut set = JoinSet::new();
            set.spawn(async move { Self::handle_client(reader, sender).await });
            set.spawn(async move { Self::keep_alive(keep_alive_sender).await });
            set.spawn(async move {
                while let Some(recv) = receiver.recv().await {
                    let buffer = match recv {
                        OutgoingPacket::KeepAlive(x) => x.to_bytes().await,
                        OutgoingPacket::PlayerListResponse(x) => x.to_bytes().await,
                        OutgoingPacket::Status(x) => x.to_bytes().await,
                        OutgoingPacket::LoginSuccess(x) => x.to_bytes().await,
                        OutgoingPacket::RegistryData(x) => x.to_bytes().await,
                        OutgoingPacket::FinishConfiguration(x) => x.to_bytes().await,
                        OutgoingPacket::ChunkDataUpdateLight(x) => x.to_bytes().await,
                        OutgoingPacket::GameEvent(x) => x.to_bytes().await,
                        OutgoingPacket::SynchronizePlayerPosition(x) => x.to_bytes().await,
                        OutgoingPacket::PlayLogin(x) => x.to_bytes().await,
                        OutgoingPacket::SetDefaultSpawnPosition(x) => x.to_bytes().await,
                        OutgoingPacket::PlayDisconnect(x) => x.to_bytes().await,
                    }
                    .unwrap();

                    writer.write_all(&buffer[..]).await.unwrap();
                }
            });

            while let Some(_) = set.join_next().await {
                drop(set);
                break;
            }
        }
    }

    /// This thread handles the main logic of a connected client
    async fn handle_client(mut reader: OwnedReadHalf, sender: Sender<OutgoingPacket>) {
        println!("{} connected", reader.peer_addr().unwrap());
        let mut gameplay_state = GameplayState::None;
        let mut ingame_state = IngameState::Config;

        loop {
            let len = reader.read_var_i32().await.unwrap_or(0) as usize;

            if len == 0 {
                println!("{} disconnected", reader.peer_addr().unwrap());
                break;
            }

            let mut packet_buffer: Vec<u8> = vec![0u8; len];
            reader.read_exact(&mut packet_buffer).await.unwrap();

            let mut cursor = Cursor::new(packet_buffer);
            let packet_id = cursor.read_var_i32().await.unwrap();

            match gameplay_state {
                GameplayState::None => Handshake::new(&mut cursor, &mut gameplay_state, &sender).await,
                GameplayState::Status => Status::new(&sender).await,
                GameplayState::Login => Login::new(&mut cursor, &mut gameplay_state, &sender).await,
                GameplayState::LoginAcknowledge => LoginAcknowledge::new(&mut gameplay_state, &sender).await,
                GameplayState::Play => match ingame_state {
                    IngameState::Config => match packet_id {
                        0x00 => {
                            let info = ClientInformation::receive(&mut cursor).await.unwrap();
                            println!("{info:?}");
                        }
                        0x01 => {
                            let msg = PluginMessage::receive(&mut cursor).await.unwrap();
                            println!("{msg:?}");
                        }
                        0x02 => {
                            FinishConfiguration::receive(&mut cursor).await.unwrap();
                            ingame_state = IngameState::Playing;
                            println!("[Config] Finishing configuration");

                            sender.send(OutgoingPacket::PlayLogin(PlayLogin::default())).await.unwrap();
                            sender
                                .send(OutgoingPacket::ChunkDataUpdateLight(ChunkDataUpdateLight::default()))
                                .await
                                .unwrap();
                            sender
                                .send(OutgoingPacket::SynchronizePlayerPosition(SynchronizePlayerPosition::default()))
                                .await
                                .unwrap();
                            sender.send(OutgoingPacket::GameEvent(GameEvent::default())).await.unwrap();
                            sender
                                .send(OutgoingPacket::SetDefaultSpawnPosition(SetDefaultSpawnPosition::default()))
                                .await
                                .unwrap();
                        }

                        _ => {
                            println!("len_{len} packetId_{packet_id}");
                            println!("{}", String::from_utf8_lossy(&cursor.into_inner()).to_string())
                        }
                    },
                    IngameState::Playing => match packet_id {
                        // Chat Message
                        0x05 => {
                            let message = cursor.read_string(256).await.unwrap();
                            println!("Message: {message}");
                        }

                        // KeepAlive Response
                        0x15 => {}

                        // Player Position
                        0x17 => {}

                        // Player Position and Rotation
                        0x18 => {}

                        // Player Rotation
                        0x19 => {}

                        _ => {
                            println!("len_{len} packetId_{packet_id}");
                            println!("{}", String::from_utf8_lossy(&cursor.into_inner()).to_string())
                        }
                    },
                },

                _ => {
                    println!("len_{len} packetId_{packet_id}");
                    println!("{}", String::from_utf8_lossy(&cursor.into_inner()).to_string())
                }
            }
        }
    }

    /// This thread sents a KeepAlive packet every 15 seconds
    async fn keep_alive(sender: Sender<OutgoingPacket>) {
        println!("[KeepAlive] Started");

        let mut interval_timer = time::interval(Duration::from_secs(15));
        interval_timer.tick().await;

        loop {
            interval_timer.tick().await;

            let packet = KeepAlive::new(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64);
            sender.send(OutgoingPacket::KeepAlive(packet)).await.unwrap();
        }
    }
}
