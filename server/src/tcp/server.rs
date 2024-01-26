use crate::decoder::{DecoderReadExt, ReceiveFromStream};
use crate::encoder::SendToStream;
use crate::packets::chunk::{ChunkDataUpdateLight, SetDefaultSpawnPosition, SynchronizePlayerPosition};
use crate::packets::config::{ClientInformation, ReceiveFinishConfiguration, ServerboundPluginMessage};
use crate::packets::event::GameEvent;
use crate::packets::incoming::handshake::HandShake;
use crate::packets::incoming::keep_alive_response::KeepAliveResponse;
use crate::packets::incoming::player_position::PlayerPosition;
use crate::packets::incoming::player_position_rotation::PlayerPositionRotation;
use crate::packets::incoming::player_rotation::PlayerRotation;
use crate::packets::login::LoginAcknowledge;
use crate::packets::outgoing::keep_alive::KeepAlive;
use crate::packets::play::PlayLogin;
use crate::packets::{login::Login, status::Status};
use crate::player::mc_player::McPlayer;
use futures::future::join;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{
    fmt::Debug,
    io::{Cursor, Read},
    net::{TcpListener, TcpStream, ToSocketAddrs},
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::{task, time};

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

pub struct TcpServer {
    players: Arc<Mutex<Vec<McPlayer>>>,
}

impl TcpServer {
    pub fn new() -> Self {
        Self {
            players: Arc::new(Mutex::new(vec![])),
        }
    }

    pub async fn start<E>(&self, endpoint: &E) -> std::io::Result<()>
    where
        E: ToSocketAddrs + Debug,
    {
        let listener = TcpListener::bind(endpoint)?;
        println!("Server started @ {endpoint:?}");

        for stream in listener.incoming() {
            let players = self.players.clone();

            match stream {
                Ok(stream) => {
                    let mut cloned_stream = stream.try_clone()?;

                    let connection_task = task::spawn(async move { Self::handle_connection(players.clone(), stream) });
                    let keep_alive_task = task::spawn(async move {
                        println!("Starting KeepAlive thread...");

                        let mut interval_timer = time::interval(Duration::from_secs(15));
                        interval_timer.tick().await;

                        loop {
                            interval_timer.tick().await;
                            println!("[KeepAlive] sending to player...");
                            KeepAlive::new(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64)
                                .send(&mut cloned_stream)
                                .unwrap();
                        }
                    });

                    join(connection_task, keep_alive_task).await;
                }
                Err(_) => {
                    eprintln!("There was an error while accepting the incoming connection!");
                }
            };
        }

        Ok(())
    }

    fn handle_connection(players: Arc<Mutex<Vec<McPlayer>>>, mut stream: TcpStream) {
        println!("{} connected", stream.peer_addr().unwrap());
        let mut gameplay_state = GameplayState::None;
        let mut ingame_state = IngameState::Config;

        loop {
            let len = stream.read_var_i32().unwrap_or(0) as usize;

            if len == 0 {
                println!("{} disconnected", stream.peer_addr().unwrap());
                break;
            }

            let mut packet_buffer: Vec<u8> = vec![0u8; len];
            stream.read_exact(&mut packet_buffer).unwrap();

            let mut cursor = Cursor::new(packet_buffer);
            let packet_id = cursor.read_var_i32().unwrap();

            match gameplay_state {
                GameplayState::None => HandShake::handle(&mut cursor, &mut gameplay_state, &mut stream),
                GameplayState::Status => Status::handle(&mut stream),
                GameplayState::Login => Login::handle(&mut cursor, players.clone(), &mut gameplay_state, &mut stream),
                GameplayState::LoginAcknowledge => LoginAcknowledge::handle(&mut stream, &mut gameplay_state),
                GameplayState::Play => match ingame_state {
                    IngameState::Config => match packet_id {
                        0x00 => {
                            let client_information = ClientInformation::receive(&mut cursor).unwrap();
                            println!("{:?}", client_information);
                        }
                        0x01 => {
                            let plugin_response = ServerboundPluginMessage::receive(&mut cursor).unwrap();
                            println!("{:?}", plugin_response);
                        }
                        0x02 => {
                            ReceiveFinishConfiguration::receive(&mut cursor).unwrap();
                            println!("[Config] Finishing configuration");
                            ingame_state = IngameState::Playing;

                            PlayLogin::default().send(&mut stream).unwrap();
                            ChunkDataUpdateLight::default().send(&mut stream).unwrap();
                            SynchronizePlayerPosition::default().send(&mut stream).unwrap();
                            GameEvent::default().send(&mut stream).unwrap();
                            SetDefaultSpawnPosition::default().send(&mut stream).unwrap();
                        }
                        _ => {
                            println!("len_{len} packetId_{packet_id}");
                            println!("{}", String::from_utf8_lossy(&cursor.into_inner()).to_string())
                        }
                    },
                    IngameState::Playing => match packet_id {
                        // KeepAlive Response
                        0x15 => {
                            let res = KeepAliveResponse::receive(&mut cursor).unwrap();
                            println!("KeepAlive response: {}", res.id);
                        }

                        // Player Position
                        0x17 => {
                            let pos = PlayerPosition::receive(&mut cursor).unwrap();
                            println!("{},{},{} [{}]", pos.x, pos.y, pos.z, pos.on_ground);
                        }

                        // Player Position and rotation
                        0x18 => {
                            let pos_rot = PlayerPositionRotation::receive(&mut cursor).unwrap();
                            println!(
                                "{},{},{} | {} | {} | [{}]",
                                pos_rot.x, pos_rot.y, pos_rot.z, pos_rot.yaw, pos_rot.pitch, pos_rot.on_ground
                            );
                        }

                        // Player rotation
                        0x19 => {
                            let rot = PlayerRotation::receive(&mut cursor).unwrap();
                            println!("{} | {} | [{}]", rot.yaw, rot.pitch, rot.on_ground);
                        }

                        _ => {
                            println!("len_{len} packetId_{packet_id}");
                            println!("{}", String::from_utf8_lossy(&cursor.into_inner()).to_string())
                        }
                    },
                },
            }
        }
    }
}
