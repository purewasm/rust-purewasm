#![cfg_attr(not(test), no_std)]

extern crate alloc;
pub mod codec;
pub mod memory;
pub use lol_alloc;
pub use serde;
pub type PureResult<T> = Result<T, purewasm_error::PureError>;
pub use purewasm_bindgen::purewasm_bindgen;
#[macro_export]
macro_rules! use_purewasm {
    () => {
        extern crate alloc;
        use purewasm_error::PureError;
        use alloc::{boxed::Box, vec::Vec};
        use $crate::serde::{de::DeserializeOwned, Serialize};
        use $crate::{codec::Codec, memory::WasmMemory, PureResult};

        #[cfg(target_arch = "wasm32")]
        use $crate::lol_alloc::{AssumeSingleThreaded, FreeListAllocator};

        #[cfg(target_arch = "wasm32")]
        #[global_allocator]
        static ALLOCATOR: AssumeSingleThreaded<FreeListAllocator> =
            unsafe { AssumeSingleThreaded::new(FreeListAllocator::new()) };

        #[cfg(not(test))]
        #[panic_handler]
        fn panic(_info: &core::panic::PanicInfo) -> ! {
            loop {}
        }

        #[no_mangle]
        pub extern "C" fn alloc(len: usize) -> *mut u8 {
            let mut byte_array: Vec<u8> = Vec::with_capacity(len);
            let ptr = byte_array.as_mut_ptr();
            core::mem::forget(ptr);
            ptr
        }

        #[no_mangle]
        pub extern "C" fn de_alloc(ptr: *mut u8) {
            unsafe {
                drop(Box::from_raw(ptr));
            }
        }
    };
}
