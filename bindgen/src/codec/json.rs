#![cfg_attr(not(test), no_std)]
extern crate alloc;
use alloc::{format, vec::Vec};
use crate::error::WasmError;
use serde::{de::DeserializeOwned, Serialize};

use crate::codec::Codec;

pub struct JsonCodec;

impl Codec for JsonCodec {
    fn code(&self) -> i64 {
        0x0200
    }

    fn to_bytes<T: Serialize>(&self, t: &T) -> Result<Vec<u8>, WasmError> {
        let r = serde_json::to_vec(t);
        match r {
            Ok(t) => Ok(t),
            Err(e) => Err(WasmError::new("JSON_SERIALIZE_ERROR", &format!("{:?}", e))),
        }
    }

    fn from_bytes<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, WasmError> {
        match serde_json::from_slice(bytes) {
            Ok(t) => Ok(t),
            Err(e) => Err(WasmError::new(
                "JSON_DESERIALIZE_ERROR",
                &format!("{:?}", e),
            )),
        }
    }
}
