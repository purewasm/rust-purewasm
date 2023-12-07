#![allow(warnings)]
#![cfg_attr(not(test), no_std)]
extern crate alloc;

pub mod codec;
pub mod memory;
pub use lol_alloc;
pub use purewasm_proc_macro::purewasm_bindgen;
pub use serde;

#[macro_export]
macro_rules! get {
    ($key:expr) => {{
        unsafe {
            let memory = WasmMemory {
                codec: purewasm::DefaultCodec,
            };
            let (result_ptr, result_len) = get_from_db($key.as_ptr() as i32, $key.len() as i32);
            memory.from_memory(result_ptr as *mut u8, result_len)
        }
    }};
}

#[macro_export]
macro_rules! put {
    ($key:expr, $value: expr) => {{
        unsafe {
            let memory = WasmMemory {
                codec: purewasm::DefaultCodec,
            };
            let (value_ptr, value_len) = memory.to_memory($value);
            put_to_db($key.as_ptr() as i32, $key.len() as i32, value_ptr, value_len);
        }
    }};
}

pub mod prelude {
    pub use crate::memory::WasmMemory;
    pub use crate::purewasm_bindgen;
    pub use crate::codec::Codec;
    pub use crate::*;
    pub use alloc::{
        boxed::Box,
        string::{String, ToString},
        vec::Vec,
    };
    pub use serde::de::DeserializeOwned;

    pub fn get_from_host<T: DeserializeOwned>(key: &str) -> Result<T, String> {
        //let (result_ptr, )get(key.as_ptr() as i32, key.len() as i32);
        todo!()
    }
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
        pub fn get_from_db(key_ptr: i32, key_len: i32) -> (i32, i32);
        pub fn put_to_db(key_ptr: i32, key_len: i32, value_ptr: i32, value_len: i32);
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
