#![no_main]
#![cfg_attr(not(test), no_std)]
extern crate alloc;
use purewasm::bindgen::prelude::*;

use serde::{Serialize, Deserialize};
use alloc::{format, string::String};

#[derive(Debug, Serialize, Deserialize)]
pub struct Input {
    pub code: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomResult {
    pub msg: String,
}

#[purewasm_bindgen]
pub fn handle_example(input: Input) -> Result<CustomResult, String> {
    Ok(CustomResult {
        msg: format!("The input code is {}", input.code),
    })
}