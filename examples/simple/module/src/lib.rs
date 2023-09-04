#![cfg_attr(not(test), no_main)]
#![cfg_attr(not(test), no_std)]


use purewasm_bindgen::purewasm_bindgen;
use purewasm_core::use_purewasm;
use purewasm_simple_shared::{Input, CustomResult};


use_purewasm!(purewasm_core, cbor);

#[purewasm_bindgen]
pub fn example(input: Input) -> PureResult<CustomResult> {
    purewasm_simple_shared::example(input)
}
