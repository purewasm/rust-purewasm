#![no_main]
#![cfg_attr(not(test), no_std)]
extern crate alloc;
use purewasm::bindgen::prelude::*;


use serde::{Serialize, Deserialize};
use alloc::{format, string::String, borrow::ToOwned};

#[derive(Debug, Serialize, Deserialize)]
pub struct Input {
    pub code: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomResult {
    pub msg: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomError {
    pub msg: String,
}

impl From<&str> for CustomError {
    fn from(s: &str) -> Self {
        Self {
            msg: s.to_owned()
        }
    }
}

impl From<String> for CustomError {
    fn from(s: String) -> Self {
        Self {
            msg: s
        }
    }
}

#[purewasm_bindgen]
pub fn handle_example(input: Input) -> Result<CustomResult, CustomError> {
    //let codec = purewasm::DefaultCodec;
    //let t: String = purewasm::DefaultCodec::from_bytes("".as_bytes())?;
    let a: CustomResult = get!("")?;
    put!("aa", a);
    //let a: String = get_from_host("").unwrap();
    Ok(CustomResult {
        msg: format!("The input code is {}", input.code),
    })
}