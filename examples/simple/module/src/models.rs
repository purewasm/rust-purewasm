use alloc::{borrow::ToOwned, string::String};
use purewasm_core::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "purewasm_core::serde")]
pub struct Input {
    pub code: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "purewasm_core::serde")]
pub struct CustomResult {
    pub msg: String,
}
