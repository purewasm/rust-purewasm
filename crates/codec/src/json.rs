#[cfg(feature = "json")]
use alloc::vec::Vec;
use serde::{de::DeserializeOwned, Serialize};

use crate::error::PureError;

use super::Codec;
pub struct JsonCodec;

impl Codec for JsonCodec {
    fn get_code(&self) -> i64 {
        0x0200
    }

    fn to_bytes<T: Serialize>(&self, t: T) -> Result<Vec<u8>, PureError> {
        let r = serde_json::to_vec(&t);
        match r {
            Ok(t) => Ok(t),
            Err(_) => Err(PureError::new("JSON_SERIALIZE_ERROR")),
        }
    }

    fn from_bytes<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, PureError> {
        match serde_json::from_slice(bytes) {
            Ok(t) => Ok(t),
            Err(_) => Err(PureError::new("JSON_DESERIALIZE_ERROR")),
        }
    }
}

/*
    #[test]
    fn json_test(){
        let codec = json::JsonCodec;
        let bytes = codec.to_bytes(Input{code:  3}).unwrap();
        let abc: Input = json::JsonCodec::from_bytes(&bytes).unwrap();
        eprintln!("{:?}", abc);
    }
*/