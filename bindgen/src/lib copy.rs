#![cfg_attr(not(test), no_std)]
#![allow(warnings)]
extern crate alloc;

mod codec;
mod error;
mod host;
mod memory;

pub use lol_alloc;
pub use serde;
pub use wasmledger_bindgen_macro::wasmledger_bindgen;

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
    pub use crate::error::WasmError;
    pub use alloc::{
        boxed::Box,
        string::{String, ToString},
        vec::Vec,
    };
    pub use crate::{codec::Codec, memory::WasmMemory};

    cfg_if::cfg_if! {
        if #[cfg(feature = "json")]{
            pub use crate::codec::json::JsonCodec as CodecImpl;
        }else if #[cfg(feature = "cbor")]{
            pub use crate::codec::cbor::CborCodec as CodecImpl;
        }else {
            compile_error!("Please enable one of the following features: cbor, json");
        }
    }
    pub use crate::wasmledger_bindgen;
    pub use crate::{get, put};

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

    /*pub fn log_error() {
            unsafe {
                error(0, 0);
            }
        }

        pub fn get_from_host<T: DeserializeOwned>(key: &str) -> Result<Option<T>, WasmError> {
            unsafe {
                let memory = WasmMemory {
                    codec: CodecImpl {},
                };
                let (result_ptr, result_len) = get(key.as_ptr() as i32, key.len() as i32);
                let result: Option<T> = if result_len == 0 {
                    None
                } else {
                    let value = memory.from_memory(result_ptr as *mut u8, result_len)?;
                    Some(value)
                };
                Ok(result)
            }
        }

        pub fn put_to_host<T: Serialize>(key: &str, value: T) {

        }

        pub struct Host;

        impl Host {
            pub fn get<T: DeserializeOwned>(&self, key: &str) -> Option<T> {
                let result = unsafe{
                    get(0,0)
                };
                None
            }
        }


    pub trait Hosta {
        fn get(&self, key: &str) -> Option<Vec<u8>>;
        fn get_signers(&self) -> Vec<String>;
    }

    pub fn verify<H: Hosta>(host: H, key: &str) {
        let _ = host.get("key");
    }*/
}
