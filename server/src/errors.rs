use std::{io::Error, string::FromUtf8Error};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DecodeError {
    #[error("Error while reading data")]
    IOError(#[from] Error),
    #[error("Boolean are parsed from byte. Valid byte value are 0 or 1.")]
    NonBoolValue,
    #[error("String length can't be more than provided value.")]
    StringTooLong { length: usize, max_length: u16 },
    #[error("Byte array was not recognized as valid UTF-8 string.")]
    FromUtf8Error(#[from] FromUtf8Error),
    #[error("VarInt is too long")]
    VarIntTooLong { max_bytes: u32 },
}

#[derive(Debug, Error)]
pub enum EncodeError {
    #[error("Error while writing data")]
    IOError(#[from] Error),
    #[error("String length can't be more than provided value.")]
    StringTooLong { length: usize, max_length: u16 },
}
