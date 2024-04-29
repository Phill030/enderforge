use macros::Streamable;

#[derive(Streamable, Default)]
#[packet_id(0x02)]
pub struct FinishConfiguration {}
