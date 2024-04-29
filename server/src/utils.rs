use crate::{encoder::EncoderWriteExt, types::VarInt};
use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};

pub const MAX_STRING_LEN: u16 = 32767;

pub fn system_time_millis() -> Result<u64, SystemTimeError> {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH)?;
    let time_in_ms = since_the_epoch.as_secs() * 1000 + u64::from(since_the_epoch.subsec_nanos()) / 1_000_000;

    Ok(time_in_ms)
}

#[must_use]
pub async fn prepare_response(packet_id: VarInt, mut data: Vec<u8>) -> Vec<u8> {
    let mut temp_buffer = vec![];
    temp_buffer.write_var_i32(packet_id).await.unwrap();
    temp_buffer.append(&mut data);

    let mut buffer = Vec::new();
    buffer.write_var_i32(VarInt(temp_buffer.len() as i32)).await.unwrap();
    buffer.append(&mut temp_buffer);

    buffer
}
