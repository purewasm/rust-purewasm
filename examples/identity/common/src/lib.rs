#![cfg_attr(not(test), no_std)]

extern crate alloc;
pub mod wots;
pub use sha2;
ue alloc::vec::Vec;
use serde::{Serialize, Deserialize};

pub type IdEvent = Event<>
#[derive(Serialize, Deserialize)]
pub struct IdEvent {
    // The time of chain, it is not real timestamp
    timestamp: u128,
    /// Semantic version of wasm
    version: u32,
    /// Hash of wasm(context)
    version_id: Vec<u8>,
    /// Encoded event payload
    payload: Vec<u8>
}

pub enum State {
    // Sha256
    Sha256([u8; 32]),
    // Sha512
    Sha512([u8; 64])
}
 