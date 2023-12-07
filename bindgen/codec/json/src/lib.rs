#![cfg_attr(not(test), no_std)]
extern crate alloc;
use alloc::{string::String, vec::Vec};
use purewasm_bindgen::codec::Codec;
use serde::{de::DeserializeOwned, Serialize};

pub struct JsonCodec;

impl Codec for JsonCodec {
    fn code(&self) -> i64 {
        0x0200
    }

    fn to_bytes<T: Serialize>(&self, t: &T) -> Result<Vec<u8>, String> {
        let r = serde_json::to_vec(t);
        match r {
            Ok(t) => Ok(t),
            Err(_) => Err("JSON_SERIALIZE_ERROR".into()),
        }
    }

    fn from_bytes<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, String> {
        match serde_json::from_slice(bytes) {
            Ok(t) => Ok(t),
            Err(_) => Err("JSON_DESERIALIZE_ERROR".into()),
        }
    }
}
