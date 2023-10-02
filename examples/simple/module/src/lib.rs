#![no_main]
#![cfg_attr(not(test), no_std)]

use purewasm_simple_core::*;
use purewasm::bindgen::prelude::*;

purewasm_setup!();

#[purewasm_bindgen]
pub fn handle_example(input: Input) -> PureResult<CustomResult> {
    purewasm_simple_core::handle_example(input)
}