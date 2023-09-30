#![cfg_attr(not(test), no_std)]

extern crate alloc;
use alloc::{format, string::String, vec::Vec};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Input {
    pub code: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomResult {
    pub msg: String,
}

pub fn example(input: Input) -> purewasm::PureResult<CustomResult> {
    Ok(CustomResult {
        msg: format!("The input code is {}", input.code),
    })
}
