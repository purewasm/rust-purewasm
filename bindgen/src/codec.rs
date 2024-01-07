use alloc::vec::Vec;
use serde::{de::DeserializeOwned, Serialize};

use crate::error::WasmError;

pub trait Codec {
    fn code(&self) -> i64;

    fn to_bytes<T: Serialize>(&self, t: &T) -> Result<Vec<u8>, WasmError>;

    fn from_bytes<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, WasmError>;
}

#[cfg(feature = "json")]
pub(crate) mod json;

#[cfg(feature = "cbor")]
pub(crate) mod cbor;