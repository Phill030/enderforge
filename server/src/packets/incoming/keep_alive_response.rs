use crate::decoder::Decoder;
use macros::Receivable;

#[derive(Receivable)]
pub struct KeepAliveResponse {
    pub id: i64,
}
