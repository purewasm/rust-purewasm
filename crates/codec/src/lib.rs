#![cfg_attr(not(test), no_std)]
extern crate alloc;
use alloc::vec::Vec;
use purewasm_model::PureError;
use serde::{de::DeserializeOwned, Serialize};
pub mod cbor;

pub trait Codec {
    fn get_code(&self) -> i64;
    
    fn to_bytes<T: Serialize>(&self, t: &T) -> Result<Vec<u8>, PureError>;

    fn from_bytes<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, PureError>;
}
