use alloc::vec::Vec;
use serde::{de::DeserializeOwned, Serialize};

use purewasm_core::error::WasmError;

pub trait Codec {
    fn code(&self) -> i64;

    fn to_bytes<T: Serialize>(&self, t: &T) -> Result<Vec<u8>, WasmError>;

    fn from_bytes<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, WasmError>;
}
