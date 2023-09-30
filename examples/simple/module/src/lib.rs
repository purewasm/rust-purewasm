#![no_main]
#![cfg_attr(not(test), no_std)]

use purewasm_simple_shared::*;
use purewasm::bindgen::prelude::*;

purewasm_setup!();

#[purewasm_bindgen]
pub fn example(input: Input) -> PureResult<CustomResult> {
    purewasm_simple_shared::example(input)
}