#![cfg_attr(not(test), no_std)]

extern crate alloc;
use serde::{Deserialize, Serialize};

pub type PureResult<T> = Result<T, PureError>;
pub use serde;

pub enum ByteValue {
    ThirtyTwo([u8; 32]),
    SixtyFour([u8; 64]),
}

pub struct PureEvent<P, R, S> {
    pub wam_id: ByteValue,
    pub payload: P,
    pub result: R,
    pub state: S,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum ErrorValue {
    Boolean(bool),
    Number(i32),
    String(&'static str),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PureError {
    code: &'static str,
    details: Option<[(&'static str, ErrorValue); 4]>,
}

impl PureError {
    pub fn new(code: &'static str) -> Self {
        Self {
            code: code,
            details: None,
        }
    }
}
