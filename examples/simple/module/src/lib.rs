#![no_std]
#![no_main]

pub mod models;
use crate::models::*;
use alloc::format;
use purewasm_core::use_purewasm;
use purewasm_bindgen::purewasm_bindgen;

use_purewasm!(purewasm_core, cbor);

#[purewasm_bindgen]
pub fn example(input: Input) -> PureResult<CustomResult> {
   Ok(CustomResult{msg: format!("The input code is {}", input.code)})
}
