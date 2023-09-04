#![cfg_attr(not(test), no_main)]
#![cfg_attr(not(test), no_std)]

extern crate alloc;
pub mod wots;
pub use sha2;
use alloc::vec::Vec;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct IdEvent {
    /// Semantic version of wasm
    version: u32,
    /// Hash of wasm(context)
    version_id: Vec<u8>,
    /// Event payload
    payload: Vec<u8>
}