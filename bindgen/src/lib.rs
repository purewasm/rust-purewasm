#![cfg_attr(not(test), no_std)]
#![allow(warnings)]
extern crate alloc;

mod codec;
#[cfg(feature = "cbor")]
mod cbor;
#[cfg(feature = "json")]
mod json;
mod memory;
mod error;

pub use lol_alloc;
pub use purewasm_proc_macro::purewasm_bindgen;
pub use serde;

#[macro_export]
macro_rules! get {
    ($key:expr, $ty:ty) => {{
        unsafe {
            let memory = WasmMemory {
                codec: CodecImpl {},
            };
            let (result_ptr, result_len) = get($key.as_ptr() as i32, $key.len() as i32);
            let value: Option<$ty> = if result_len == 0 {
                None
            } else {
                Some(memory.from_memory(result_ptr as *mut u8, result_len)?)
            };
            value
        }
    }};
}

#[macro_export]
macro_rules! put {
    ($key:expr, $value: expr) => {{
        unsafe {
            let memory = WasmMemory {
                codec: CodecImpl {},
            };
            let (value_ptr, value_len) = memory.to_memory($value)?;
            put(
                $key.as_ptr() as i32,
                $key.len() as i32,
                value_ptr,
                value_len,
            );
        }
    }};
}

pub mod prelude {
    cfg_if::cfg_if! {
        if #[cfg(feature = "json")]{
            pub use crate::json::JsonCodec as CodecImpl;
        }else if #[cfg(feature = "cbor")]{
            pub use crate::cbor::CborCodec as CodecImpl;
        }else {
            compile_error!("Please enable one of the following features: cbor, json");
        }
    }
    pub use crate::purewasm_bindgen;
    pub use crate::{codec::Codec, get, memory::WasmMemory, put};
    pub use alloc::{
        boxed::Box,
        string::{String, ToString},
        vec::Vec,
    };
    pub use crate::error::WasmError;
    pub use serde::de::DeserializeOwned;
    // Import allocator for WebAssembly
    #[cfg(target_arch = "wasm32")]
    use crate::lol_alloc::{AssumeSingleThreaded, FreeListAllocator};

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

    extern "C" {
        pub fn get(key_ptr: i32, key_len: i32) -> (i32, i32);
        pub fn put(key_ptr: i32, key_len: i32, value_ptr: i32, value_len: i32);
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
