#![cfg_attr(not(test), no_std)]
extern crate alloc;
#[cfg(feature = "cbor")]
pub mod cbor;
#[cfg(feature = "json")]
pub mod json;

