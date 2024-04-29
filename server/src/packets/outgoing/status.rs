use crate::{encoder::Encoder, packets::packet::OutgoingPacket, utils::system_time_millis};
use macros::Streamable;
use tokio::sync::mpsc::Sender;

#[derive(Streamable)]
#[packet_id(0x01)]
pub struct Status {
    system_time_millis: u64,
}
impl Status {
    pub async fn new(sender: &Sender<OutgoingPacket>) {
        sender
            .send(OutgoingPacket::Status(Self {
                system_time_millis: system_time_millis().unwrap(),
            }))
            .await
            .unwrap();
    }
}
