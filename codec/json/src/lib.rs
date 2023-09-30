#![cfg_attr(not(test), no_std)]
extern crate alloc;
use alloc::vec::Vec;
use serde::{de::DeserializeOwned, Serialize};
use purewasm_core::{Codec, PureError};

pub struct JsonCodec;

impl Codec for JsonCodec {
    fn get_code(&self) -> i64 {
        0x0200
    }

    fn to_bytes<T: Serialize>(&self, t: &T) -> Result<Vec<u8>, PureError> {
        let r = serde_json::to_vec(t);
        match r {
            Ok(t) => Ok(t),
            Err(_) => Err("JSON_SERIALIZE_ERROR".into()),
        }
    }

    fn from_bytes<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, PureError> {
        match serde_json::from_slice(bytes) {
            Ok(t) => Ok(t),
            Err(_) => Err("JSON_DESERIALIZE_ERROR".into()),
        }
    }
}

