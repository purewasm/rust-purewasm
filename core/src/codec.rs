use alloc::vec::Vec;
use crate::error::PureError;
use serde::{de::DeserializeOwned, Serialize};

pub trait Codec {
    fn get_code(&self) -> i64;
    
    fn to_bytes<T: Serialize>(&self, t: T) -> Result<Vec<u8>, PureError>;

    fn from_bytes<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, PureError>;
}

#[cfg(feature = "json")]
pub mod json {
    use alloc::vec::Vec;
    use serde::{Serialize, de::DeserializeOwned};

    use crate::error::PureError;

    use super::Codec;
    pub struct JsonCodec;

    impl Codec for JsonCodec{
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
            match serde_json::from_slice(bytes){
                Ok(t)=> Ok(t),
                Err(_)=> Err(PureError::new("JSON_DESERIALIZE_ERROR"))
            }
        }
    }
    
}

#[cfg(feature = "cbor")]
pub mod cbor {
    use alloc::vec::Vec;
    use serde::{Serialize, de::DeserializeOwned};

    use crate::error::PureError;

    use super::Codec;

    pub struct CborCodec;

    impl Codec for CborCodec{
        fn get_code(&self) -> i64 {
           0x51
        }
    
        fn to_bytes<T: Serialize>(&self, t: T) -> Result<Vec<u8>, PureError> {
            let mut bytes: Vec<u8> = Vec::new();
            if let Err(_) = ciborium::into_writer(&t, &mut bytes){
                return Err(PureError::new("CBOR_DESERIALIZE_ERROR"));
            }
            Ok(bytes)
        }
    
        fn from_bytes<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, PureError> {
            match ciborium::from_reader(bytes) {
                Ok(t)=> Ok(t),
                Err(_)=> Err(PureError::new("CBOR_SERIALIZE_ERROR"))
            }
        }
    }
    
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use super::*;
    #[derive(Debug, Serialize, Deserialize)]
    struct Abc(i32);

    #[test]
    fn cbor_test(){
        let codec = cbor::CborCodec;
        let bytes = codec.to_bytes(Abc(3)).unwrap();
        let abc: Abc = cbor::CborCodec::from_bytes(&bytes).unwrap();
        eprintln!("{:?}", abc);
    }

    
    #[test]
    fn json_test(){
        let codec = json::JsonCodec;
        let bytes = codec.to_bytes(Abc(3)).unwrap();
        let abc: Abc = json::JsonCodec::from_bytes(&bytes).unwrap();
        eprintln!("{:?}", abc);
    }
}