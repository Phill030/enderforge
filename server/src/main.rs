use tcp::server::TcpServer;

pub mod decoder;
pub mod encoder;
pub mod errors;
pub mod packets;
mod tcp;
pub mod types;
pub mod utils;

fn main() {
    TcpServer::create(&"0.0.0.0:25565").unwrap();
}

// https://wiki.vg/Protocol#Configuration
// https://github.com/Sweattypalms/ferrumc/blob/master/crates/ferrumc_net/src/login_start.rs
// https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/procedural-macros.html
// https://gist.github.com/WinX64/3675ffee90360e9fc1e45074e49f6ede#file-registry_data-json-L370-L388
// https://nbt.mcph.to/
// !! https://github.com/PrismarineJS/minecraft-data/blob/master/data/pc/1.20/protocol.json
// !! https://wiki.vg/index.php?title=Protocol&oldid=18256
