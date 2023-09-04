#![cfg_attr(not(test), no_main)]
#![cfg_attr(not(test), no_std)]

extern crate alloc;
use serde::{Serialize, Deserialize};
use alloc::{borrow::ToOwned, collections::BTreeMap, string::String};

pub type PureResult<T> = Result<T, PureError>;
pub use serde;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum ErrorValue {
    Boolean(bool),
    Number(i32),
    String(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PureError {
    code: String,
    details: BTreeMap<String, ErrorValue>,
}

impl PureError {
    pub fn new(code: &str) -> Self {
        Self {
            code: code.to_owned(),
            details: BTreeMap::new(),
        }
    }
}
