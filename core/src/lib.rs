#![cfg_attr(not(test), no_std)]
extern crate alloc;
mod codec;
mod store;
pub use codec::Codec;
pub use store::Store;