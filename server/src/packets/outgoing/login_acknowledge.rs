use crate::{
    packets::{
        outgoing::{finish_configuration::FinishConfiguration, registry_data::RegistryData},
        packet::OutgoingPacket,
    },
    GameplayState,
};
use macros::Receivable;
use tokio::sync::mpsc::Sender;

#[derive(Receivable)]
pub struct LoginAcknowledge {}

impl LoginAcknowledge {
    pub async fn new(gameplay_state: &mut GameplayState, sender: &Sender<OutgoingPacket>) {
        println!("[LoginAck] Received");
        *gameplay_state = GameplayState::Play;

        sender
            .send(OutgoingPacket::RegistryData(RegistryData::create().await))
            .await
            .unwrap();
        sender
            .send(OutgoingPacket::FinishConfiguration(FinishConfiguration::default()))
            .await
            .unwrap();
    }
}
