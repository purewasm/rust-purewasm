#![cfg_attr(not(test), no_main)]
#![cfg_attr(not(test), no_std)]

use alloc::format;
use purewasm_bindgen::purewasm_bindgen;
use purewasm_core::use_purewasm;

use_purewasm!(purewasm_core, cbor);

#[purewasm_bindgen]
pub fn example(
    input: purewasm_simple_shared::Input,
) -> PureResult<purewasm_simple_shared::CustomResult> {
    purewasm_simple_shared::example(input)
}
