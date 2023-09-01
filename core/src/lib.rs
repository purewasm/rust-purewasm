#![no_std]
#![no_main]

extern crate alloc;
pub mod error;
pub mod codec;
pub use lol_alloc;
pub use serde;

pub const CBOR_CODEC: u64 = 0x51; 
pub const JSON_CODEC: u64 = 0x0200; 

#[macro_export]
macro_rules! use_purewasm {
    () => {
        #[cfg(target_arch = "wasm32")]
        use purewasm_core::lol_alloc::{AssumeSingleThreaded, FreeListAllocator};

        #[cfg(target_arch = "wasm32")]
        #[global_allocator]
        static ALLOCATOR: AssumeSingleThreaded<FreeListAllocator> =
            unsafe { AssumeSingleThreaded::new(FreeListAllocator::new()) };

        #[cfg(not(test))]
        #[panic_handler]
        fn panic(_info: &core::panic::PanicInfo) -> ! {
            loop {}
        }
    };
}

