#![cfg_attr(not(test), no_std)]
extern crate alloc;

use alloc::{string::String, vec::Vec};
use purewasm_bindgen::codec::Codec;
use serde::{de::DeserializeOwned, Serialize};

pub struct CborCodec;

impl Codec for CborCodec {
    fn code(&self) -> i64 {
        0x51
    }

    fn to_bytes<T: Serialize>(&self, t: &T) -> Result<Vec<u8>, String> {
        let mut bytes: Vec<u8> = Vec::new();
        if let Err(_) = ciborium::into_writer(&t, &mut bytes) {
            return Err("CBOR_SERIALIZE_ERROR".into());
        }
        Ok(bytes)
    }

    fn from_bytes<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, String> {
        match ciborium::from_reader(bytes) {
            Ok(t) => Ok(t),
            Err(_) => Err("CBOR_DESERIALIZE_ERROR".into()),
        }
    }
}

#[cfg(test)]
mod tests {

    use serde::Deserialize;

    use super::*;
    #[derive(Debug, Serialize, Deserialize)]
    struct Input {
        code: i32,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct ExampleResult {
        msg: String,
    }

    #[test]
    fn cbor_test() {
        let codec = CborCodec;
        let bytes = codec.to_bytes(&Input { code: 5 }).unwrap();
        let rbytes = codec
            .to_bytes(&ExampleResult {
                msg: "The input code is 5".to_owned(),
            })
            .unwrap();
        //let abc: Input = cbor::CborCodec::from_bytes(&bytes).unwrap();
        eprintln!("{:?}", bytes);
        eprintln!("{:?}", rbytes);
    }
}
