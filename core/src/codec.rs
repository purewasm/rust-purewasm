use alloc::vec::Vec;
use serde::{de::DeserializeOwned, Serialize};

use crate::error::PureError;

pub trait Codec {
    fn serialize<T: Serialize>(&self, t: T) -> Result<Vec<u8>, PureError>;

    fn deserialize<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, PureError>;

    fn write<T: Serialize>(&self, t: T) -> (i64, i64) {
        let r = self.serialize(t);
        let bytes = match r {
            Ok(bytes) => bytes,
            Err(e) => self.serialize(e).unwrap(),
        };
        let output_ptr = bytes.as_ptr();
        let output_len = bytes.len() as i64;
        (output_ptr as i64, output_len)
    }
}
