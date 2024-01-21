use crate::decoder::{DecoderReadExt, ReceiveFromStream};
use crate::packets::config::{ClientInformation, ReceiveFinishConfiguration, ServerboundPluginMessage};
use crate::packets::login::LoginAcknowledge;
use crate::packets::{handshake::HandShake, login::Login, status::Status};
use std::{
    fmt::Debug,
    io::{Cursor, Read},
    net::{TcpListener, TcpStream, ToSocketAddrs},
};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum GameplayState {
    None = -1,
    Status = 1,
    Login = 2,
    LoginAcknowledge = 3,
    Play = 4,
}

pub struct TcpServer;
impl TcpServer {
    pub fn create<E>(endpoint: &E) -> std::io::Result<()>
    where
        E: ToSocketAddrs + Debug,
    {
        let listener = TcpListener::bind(endpoint)?;
        println!("Server started @ {endpoint:?}");

        let runtime_builder = tokio::runtime::Builder::new_multi_thread().enable_all().build()?;
        for stream in listener.incoming() {
            match stream {
                Ok(s) => {
                    runtime_builder.spawn(async move { Self::handle_connection(s) });
                }
                Err(_) => {
                    eprintln!("There was an error while accepting the incoming connection!");
                }
            };
        }

        Ok(())
    }

    fn handle_connection(mut stream: TcpStream) {
        println!("{} connected", stream.peer_addr().unwrap());
        let mut state = GameplayState::None;

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

            match state {
                GameplayState::None => HandShake::handle(&mut cursor, &mut state, &mut stream),
                GameplayState::Status => Status::handle(&mut stream),
                GameplayState::Login => Login::handle(&mut cursor, &mut state, &mut stream),
                GameplayState::LoginAcknowledge => LoginAcknowledge::handle(&mut state, &mut stream),
                GameplayState::Play => match packet_id {
                    0x00 => {
                        // let client_information = ClientInformation::receive(&mut cursor).unwrap();
                        // println!("{:?}", client_information);
                    }
                    0x01 => {
                        // let plugin_response = ServerboundPluginMessage::receive(&mut cursor).unwrap();
                        // println!("{:?}", plugin_response);
                    }
                    0x02 => {
                        // ReceiveFinishConfiguration::receive(&mut cursor).unwrap();
                        // println!("[Config] Finishing configuration");
                    }
                    _ => {
                        println!("len_{len} packetId_{packet_id}");
                        println!("{}", String::from_utf8_lossy(&cursor.into_inner()).to_string())
                    }
                },
            }
        }
    }
}
