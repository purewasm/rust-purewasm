#![cfg_attr(not(test), no_std)]

extern crate alloc;

pub mod memory;
pub use lol_alloc;
pub use proc_macro::purewasm_bindgen;
pub use purewasm_core;
#[cfg(feature = "cbor")]
pub use purewasm_cbor;
#[cfg(feature = "json")]
pub use purewasm_json;
pub use serde;

pub mod prelude {
    pub use crate::{purewasm_bindgen, purewasm_setup};
}
// Macro to set up the environment
#[macro_export]
macro_rules! purewasm_setup {
    () => {
        extern crate alloc;

        // Import necessary items
        use alloc::{boxed::Box, vec::Vec};
        use $crate::memory::WasmMemory;
        use $crate::purewasm_core::{Codec, PureError, PureResult};
        use $crate::serde::{de::DeserializeOwned, Serialize};
        //#[cfg(not(feature = "json"))]
        use $crate::purewasm_cbor::CborCodec as DefaultCodec;
        //#[cfg(feature = "json")]
        //use $crate::purewasm_json::JsonCodec as DefaultCodec;

        // Import allocator for WebAssembly
        #[cfg(target_arch = "wasm32")]
        use $crate::lol_alloc::{AssumeSingleThreaded, FreeListAllocator};

        // Set the global allocator for WebAssembly
        #[cfg(target_arch = "wasm32")]
        #[global_allocator]
        static ALLOCATOR: AssumeSingleThreaded<FreeListAllocator> =
            unsafe { AssumeSingleThreaded::new(FreeListAllocator::new()) };

        // Panic handler for release builds
        #[cfg(not(test))]
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
    };
}
