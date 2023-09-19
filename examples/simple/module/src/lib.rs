#![no_main]
#![cfg_attr(not(test), no_std)]

use purewasm_simple_shared::{CustomResult, Input};
use purewasm_bindgen::{purewasm_bindgen, purewasm_setup};

purewasm_setup!();

use purewasm_codec::cbor::CborCodec as DefaultCodec;

#[purewasm_bindgen]
pub fn example(input: Input) -> PureResult<CustomResult> {
    purewasm_simple_shared::example(input)
}