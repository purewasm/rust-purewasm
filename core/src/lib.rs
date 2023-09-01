#![no_std]
#![no_main]

extern crate alloc;
pub mod error;

pub use lol_alloc;
pub use serde;

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
