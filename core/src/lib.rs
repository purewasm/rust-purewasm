#![cfg_attr(not(test), no_main)]
#![cfg_attr(not(test), no_std)]

extern crate alloc;
pub mod codec;
pub mod error;
pub mod memory;
pub use lol_alloc;
pub use serde;
pub type PureResult<T> = Result<T, crate::error::PureError>;

#[macro_export]
macro_rules! use_purewasm {
    ($crate_name: ident) => {
        $crate_name::use_purewasm!($crate_name, json, JsonCodec);
    };
    ($crate_name: ident, cbor) => {
        $crate_name::use_purewasm!($crate_name, cbor, CborCodec);
    };
    ($crate_name: ident, $codec_mod: ident, $codec_name: ident ) => {
        extern crate alloc;
        use alloc::{boxed::Box, vec::Vec};
        use $crate_name::codec::$codec_mod::$codec_name as DefaultCodec;
        use $crate_name::codec::Codec;
        use $crate_name::memory::WasmMemory;
        use $crate_name::serde::{de::DeserializeOwned, Serialize};
        use $crate_name::{error::PureError, PureResult};

        #[cfg(target_arch = "wasm32")]
        use $crate_name::lol_alloc::{AssumeSingleThreaded, FreeListAllocator};

        #[cfg(target_arch = "wasm32")]
        #[global_allocator]
        static ALLOCATOR: AssumeSingleThreaded<FreeListAllocator> =
            unsafe { AssumeSingleThreaded::new(FreeListAllocator::new()) };

        #[cfg(not(test))]
        #[panic_handler]
        fn panic(_info: &core::panic::PanicInfo) -> ! {
            loop {}
        }

        pub fn to_memory<T: Serialize>(t: T) -> (i64, i64) {
            let memory = WasmMemory {
                codec: DefaultCodec,
            };
            memory.to_memory(t)
        }

        pub unsafe fn from_memory<T: DeserializeOwned>(
            ptr: *mut u8,
            len: i64,
        ) -> Result<T, PureError> {
            let memory = WasmMemory {
                codec: DefaultCodec,
            };
            memory.from_memory(ptr, len)
        }

        #[no_mangle]
        pub extern "C" fn get_codec() -> i64 {
            DefaultCodec.get_code()
        }

        #[no_mangle]
        pub extern "C" fn de_alloc(ptr: *mut u8) {
            unsafe {
                Box::from_raw(ptr);
            }
        }
        // For passing input
        #[no_mangle]
        pub extern "C" fn alloc(len: usize) -> *mut u8 {
            let mut byte_array: Vec<u8> = Vec::with_capacity(len); // Replace with your byte array data
            let ptr = byte_array.as_mut_ptr();
            core::mem::forget(ptr);
            ptr
        }
    };
}
