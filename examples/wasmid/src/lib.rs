#![no_main]
#![cfg_attr(not(test), no_std)]
use purewasm_bindgen::{purewasm_bindgen, purewasm_setup};
use wasmid_core::prelude::*;

purewasm_setup!();

#[purewasm_bindgen]
pub fn handle_cmd(input: IdCommand) -> PureResult<WrappedIdEvent> {
    handle(input)
}
