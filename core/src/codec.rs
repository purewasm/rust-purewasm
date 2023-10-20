use alloc::{string::String, vec::Vec};
use serde::{de::DeserializeOwned, Serialize};

pub trait Codec {
    fn get_code(&self) -> i64;

    fn to_bytes<T: Serialize>(&self, t: &T) -> Result<Vec<u8>, String>;

    fn from_bytes<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, String>;
}
