#![no_main]
#![cfg_attr(not(test), no_std)]

extern crate alloc;
use purewasm_cbor_core::*;
use purewasm::bindgen::prelude::*;

#[purewasm_bindgen]
pub fn handle_example(input: Input) -> Result<CustomResult, String> {
    let codec = purewasm::DefaultCodec;
    let doc = get!("permits/1");
    set_value!("permits/1", &val);
    push_event!("permits/1", &val);
    purewasm_cbor_core::handle_example(input)
}