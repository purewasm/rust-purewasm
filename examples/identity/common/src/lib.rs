#![cfg_attr(not(test), no_std)]

extern crate alloc;
pub mod wots;
pub use sha2;
ue alloc::vec::Vec;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct WrappedResult {
    wam_id: ContentId,
    result: Vec<u8> 
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ContentId {
    // Sha256
    Sha256([u8; 32]),
    // Sha512
    Sha512([u8; 64])
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum ErrorValue {
    Boolean(bool),
    Number(i32),
    String(&'static str),
}

pub type PureResult<T> = Result<T, PureError>;

#[derive(Debug, Serialize, Deserialize)]
pub struct PureError {
    code: &'static str,
    details: [Option<(&'static str, ErrorValue)>; 4],
}

impl PureError {
    pub fn new(code: &'static str) -> Self {
        Self {
            code: code,
            details: [None, None, None, None],
        }
    }
}