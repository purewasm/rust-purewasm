use alloc::{borrow::ToOwned, collections::BTreeMap, string::String};
use serde::{Deserialize, Serialize};

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
