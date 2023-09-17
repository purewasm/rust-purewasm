#![no_main]
#![cfg_attr(not(test), no_std)]
/*extern crate alloc;
use alloc::{boxed::Box, vec::Vec};
use purewasm_simple_shared::{CustomResult, Input};

use purewasm_bindgen::purewasm_bindgen;
use purewasm_module::use_purewasm;

use purewasm_core::serde::{de::DeserializeOwned, Serialize};
use purewasm_core::{PureError, PureResult};
use purewasm_module::codec::cbor::CborCodec as DefaultCodec;
use purewasm_module::codec::Codec;
use purewasm_module::memory::WasmMemory;

#[cfg(target_arch = "wasm32")]
use lol_alloc::{AssumeSingleThreaded, FreeListAllocator};

#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOCATOR: AssumeSingleThreaded<FreeListAllocator> =
    unsafe { AssumeSingleThreaded::new(FreeListAllocator::new()) };

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}*/

use purewasm_simple_shared::{CustomResult, Input};
use purewasm_core::{purewasm_bindgen, purewasm_setup};

purewasm_setup!();
use purewasm_codec::cbor::CborCodec as DefaultCodec;

#[purewasm_bindgen]
pub fn example(input: Input) -> PureResult<CustomResult> {
    purewasm_simple_shared::example(input)
}