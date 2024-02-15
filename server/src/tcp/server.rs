use crate::decoder::{DecoderReadExt, ReceiveFromStream};
use crate::encoder::SendToStream;
use crate::{
    packets::{
        chunk::{ChunkDataUpdateLight, SetDefaultSpawnPosition, SynchronizePlayerPosition},
        config::{ClientInformation, ReceiveFinishConfiguration, ServerboundPluginMessage},
        event::GameEvent,
        incoming::{
            handshake::HandShake, keep_alive_response::KeepAliveResponse, player_position::PlayerPosition,
            player_position_rotation::PlayerPositionRotation, player_rotation::PlayerRotation,
        },
        login::{Login, LoginAcknowledge},
        outgoing::{keep_alive::KeepAlive, play_disconnect::PlayDisconnect},
        play::PlayLogin,
        status::Status,
    },
    player::mc_player::McPlayer,
};
use std::{
    fmt::Debug,
    io,
    io::Cursor,
    sync::{Arc, Mutex},
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tokio::{
    io::AsyncReadExt,
    net::{TcpListener, TcpStream, ToSocketAddrs},
    task, time,
};
use uuid::Uuid;

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

// Lol
pub enum Either<T, U> {
    Either(T),
    Or(U),
}

pub trait Server {
    type Player;

    // Getter
    fn get_player_by_uuid(&self, uuid: &Uuid) -> Option<Self::Player>;
    fn get_player_by_name(&self, name: String) -> Option<Self::Player>;

    fn disconnect_player<S>(&self, identifier: Either<String, &Uuid>, reason: S) -> io::Result<bool>
    where
        S: Into<String>;
}

pub struct McServer {
    players: Arc<Mutex<Vec<McPlayer>>>,
}

// impl Server for McServer {
//     type Player = McPlayer;

//     fn get_player_by_uuid(&self, uuid: &Uuid) -> Option<Self::Player> {
//         if let Ok(players) = self.players.lock() {
//             if let Some(player) = players.iter().find(|p| p.uuid.eq(uuid)) {
//                 return Some(player.clone());
//             }
//         }
//         None
//     }

//     fn get_player_by_name(&self, name: String) -> Option<Self::Player> {
//         if let Ok(players) = self.players.lock() {
//             if let Some(player) = players.iter().find(|p| p.username.eq(&name)) {
//                 return Some(player.clone());
//             }
//         }

//         None
//     }

//     async fn disconnect_player<S>(&self, identifier: Either<String, &Uuid>, reason: S) -> io::Result<bool>
//     where
//         S: Into<String>,
//     {
//         match identifier {
//             Either::Either(name) => {
//                 if let Some(mut player) = self.get_player_by_name(name) {
//                     return Ok(player.disconnect(reason).await?);
//                 }
//             }
//             Either::Or(uuid) => {
//                 if let Some(mut player) = self.get_player_by_uuid(uuid) {
//                     return Ok(player.disconnect(reason)?);
//                 }
//             }
//         }

//         Ok(false)
//     }
// }

impl McServer {
    pub fn new() -> Self {
        Self {
            players: Arc::new(Mutex::new(vec![])),
        }
    }

    pub async fn start<E>(&self, endpoint: &E) -> std::io::Result<()>
    where
        E: ToSocketAddrs + Debug,
    {
        let listener = TcpListener::bind(endpoint).await?;
        println!("Server started @ {endpoint:?}");

        loop {
            let players = self.players.clone();
            let (socket, _) = listener.accept().await?;

            task::spawn(async move { Self::handle_connection(players.clone(), socket).await })
                .await
                .unwrap();
        }
    }

    async fn handle_keep_alive(mut stream: TcpStream) {
        println!("Starting KeepAlive thread...");

        let mut interval_timer = time::interval(Duration::from_secs(15));
        interval_timer.tick().await;

        loop {
            interval_timer.tick().await;
            println!("[KeepAlive] sending to player...");
            KeepAlive::new(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64)
                .send(&mut stream)
                .await
                .unwrap();
        }
    }

    // TODO: Refactor so this function has only the player as parameter.
    async fn handle_connection(players: Arc<Mutex<Vec<McPlayer>>>, mut stream: TcpStream) {
        println!("{} connected", stream.peer_addr().unwrap());
        let mut gameplay_state = GameplayState::None;
        let mut ingame_state = IngameState::Config;

        loop {
            let len = stream.read_var_i32().await.unwrap_or(0) as usize;

            if len == 0 {
                println!("{} disconnected", stream.peer_addr().unwrap());
                break;
            }

            let mut packet_buffer: Vec<u8> = vec![0u8; len];
            stream.read_exact(&mut packet_buffer).await.unwrap();

            let mut cursor = Cursor::new(packet_buffer);
            let packet_id = cursor.read_var_i32().await.unwrap();

            match gameplay_state {
                GameplayState::None => HandShake::handle(&mut cursor, &mut gameplay_state, &mut stream).await,
                GameplayState::Status => Status::handle(&mut stream).await,
                GameplayState::Login => Login::handle(&mut cursor, players.clone(), &mut gameplay_state, &mut stream).await,
                GameplayState::LoginAcknowledge => LoginAcknowledge::handle(&mut stream, &mut gameplay_state).await,
                GameplayState::Play => match ingame_state {
                    IngameState::Config => match packet_id {
                        0x00 => {
                            let client_information = ClientInformation::receive(&mut cursor).await.unwrap();
                            println!("{:?}", client_information);
                        }
                        0x01 => {
                            let plugin_response = ServerboundPluginMessage::receive(&mut cursor).await.unwrap();
                            println!("{:?}", plugin_response);
                        }
                        0x02 => {
                            ReceiveFinishConfiguration::receive(&mut cursor).await.unwrap();
                            println!("[Config] Finishing configuration");
                            ingame_state = IngameState::Playing;

                            //TODO Keep-Alive task should start here

                            PlayLogin::default().send(&mut stream).await.unwrap();
                            ChunkDataUpdateLight::default().send(&mut stream).await.unwrap();
                            SynchronizePlayerPosition::default().send(&mut stream).await.unwrap();
                            GameEvent::default().send(&mut stream).await.unwrap();
                            SetDefaultSpawnPosition::default().send(&mut stream).await.unwrap();
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
                            PlayDisconnect::from_text(format!("{}", message.repeat(50)))
                                .send(&mut stream)
                                .await
                                .unwrap();
                        }

                        // KeepAlive Response
                        0x15 => {
                            let res = KeepAliveResponse::receive(&mut cursor).await.unwrap();
                            println!("KeepAlive response: {}", res.id);
                        }

                        // Player Position
                        0x17 => {
                            let pos = PlayerPosition::receive(&mut cursor).await.unwrap();
                            println!("{},{},{} [{}]", pos.x, pos.y, pos.z, pos.on_ground);
                        }

                        // Player Position and rotation
                        0x18 => {
                            let pos_rot = PlayerPositionRotation::receive(&mut cursor).await.unwrap();
                            println!(
                                "{},{},{} | {} | {} | [{}]",
                                pos_rot.x, pos_rot.y, pos_rot.z, pos_rot.yaw, pos_rot.pitch, pos_rot.on_ground
                            );
                        }

                        // Player rotation
                        0x19 => {
                            let rot = PlayerRotation::receive(&mut cursor).await.unwrap();
                            println!("{} | {} | [{}]", rot.yaw, rot.pitch, rot.on_ground);
                        }

                        _ => {
                            println!("len_{len} packetId_{packet_id}");
                            println!("{}", String::from_utf8_lossy(&cursor.into_inner()).to_string())
                        }
                    },
                },
                _ => {}
            }
        }
    }
}
