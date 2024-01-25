use tcp::server::TcpServer;

pub mod decoder;
pub mod encoder;
pub mod errors;
pub mod packets;
pub mod player;
mod tcp;
pub mod types;
pub mod utils;

#[tokio::main]
async fn main() {
    TcpServer::new().start(&"0.0.0.0:25565").await.unwrap();
}

// https://github.com/Sweattypalms/ferrumc/blob/master/crates/ferrumc_net/src/login_start.rs
// https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/procedural-macros.html
// https://nbt.mcph.to/
// !! https://github.com/PrismarineJS/minecraft-data/blob/master/data/pc/1.20/protocol.json
// !! https://wiki.vg/index.php?title=Protocol&oldid=18256
