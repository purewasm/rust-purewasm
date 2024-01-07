#![cfg_attr(not(test), no_std)]
#![allow(warnings)]
extern crate alloc;

mod codec;
mod error;
mod host;
mod memory;

pub mod prelude {
    pub use crate::{codec::Codec, error::WasmError, host::*, memory::WasmMemory};
    pub use alloc::{
        boxed::Box,
        string::{String, ToString},
        vec::Vec,
    };

    cfg_if::cfg_if! {
        if #[cfg(feature = "json")]{
            pub use crate::codec::json::JsonCodec as CodecImpl;
        }else if #[cfg(feature = "cbor")]{
            pub use crate::codec::cbor::CborCodec as CodecImpl;
        }else {
            compile_error!("Please enable one of the following features: cbor, json");
        }
    }

    pub use wasmledger_bindgen_macro::wasmledger_bindgen;

    // Import allocator for WebAssembly
    #[cfg(target_arch = "wasm32")]
    use lol_alloc::{AssumeSingleThreaded, FreeListAllocator};

    // Set the global allocator for WebAssembly
    #[cfg(target_arch = "wasm32")]
    #[global_allocator]
    static ALLOCATOR: AssumeSingleThreaded<FreeListAllocator> =
        unsafe { AssumeSingleThreaded::new(FreeListAllocator::new()) };

    // Panic handler for release builds
    #[cfg(target_arch = "wasm32")]
    #[panic_handler]
    fn panic(_info: &core::panic::PanicInfo) -> ! {
        loop {}
    }

    // Allocation function for WebAssembly
    #[no_mangle]
    pub extern "C" fn alloc(len: usize) -> *mut u8 {
        let mut byte_array: Vec<u8> = Vec::with_capacity(len);
        let ptr = byte_array.as_mut_ptr();
        core::mem::forget(ptr);
        ptr
    }

    // Deallocation function for WebAssembly
    #[no_mangle]
    pub extern "C" fn de_alloc(ptr: *mut u8) {
        unsafe {
            drop(Box::from_raw(ptr));
        }
    }
}
